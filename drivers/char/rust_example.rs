// SPDX-License-Identifier: GPL-2.0

//! Rust example module

#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(test)]

use alloc::{boxed::Box, sync::Arc};
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{
    chrdev, condvar_init, cstr,
    file_operations::{File, FileOpener, FileOperations},
    miscdev, mutex_init, spinlock_init,
    sync::{CondVar, Mutex, SpinLock},
    user_ptr::{UserSlicePtrReader, UserSlicePtrWriter},
    Error,
};

module! {
    type: RustExample,
    name: b"rust_example",
    author: b"Rust for Linux Contributors",
    description: b"An example kernel module written in Rust",
    license: b"GPL v2",
    params: {
        my_bool: bool {
            default: true,
            permissions: 0,
            description: b"Example of bool",
        },
        my_i32: i32 {
            default: 42,
            permissions: 0o644,
            description: b"Example of i32",
        },
        my_str: str {
            default: b"default str val",
            permissions: 0o644,
            description: b"Example of a string param",
        },
        my_usize: usize {
            default: 42,
            permissions: 0o644,
            description: b"Example of usize",
        },
        my_array: ArrayParam<i32, 3> {
            default: [0, 1],
            permissions: 0,
            description: b"Example of array",
        },
    },
}

const MAX_TOKENS: usize = 3;

struct SharedStateInner {
    token_count: usize,
}

struct SharedState {
    state_changed: CondVar,
    inner: Mutex<SharedStateInner>,
}

impl SharedState {
    fn try_new() -> KernelResult<Arc<Self>> {
        let state = Arc::try_new(Self {
            // SAFETY: `condvar_init!` is called below.
            state_changed: unsafe { CondVar::new() },
            // SAFETY: `mutex_init!` is called below.
            inner: unsafe { Mutex::new(SharedStateInner { token_count: 0 }) },
        })?;
        // SAFETY: `state_changed` is pinned behind `Arc`.
        let state_changed = unsafe { Pin::new_unchecked(&state.state_changed) };
        kernel::condvar_init!(state_changed, "SharedState::state_changed");
        // SAFETY: `inner` is pinned behind `Arc`.
        let inner = unsafe { Pin::new_unchecked(&state.inner) };
        kernel::mutex_init!(inner, "SharedState::inner");
        Ok(state)
    }
}

struct Token {
    shared: Arc<SharedState>,
}

impl FileOpener<Arc<SharedState>> for Token {
    fn open(shared: &Arc<SharedState>) -> KernelResult<Self::Wrapper> {
        Ok(Box::try_new(Self {
            shared: shared.clone(),
        })?)
    }
}

impl FileOperations for Token {
    type Wrapper = Box<Self>;

    kernel::declare_file_operations!(read, write);

    fn read(&self, _: &File, data: &mut UserSlicePtrWriter, offset: u64) -> KernelResult<usize> {
        // Succeed if the caller doesn't provide a buffer or if not at the start.
        if data.is_empty() || offset != 0 {
            return Ok(0);
        }

        {
            let mut inner = self.shared.inner.lock();

            // Wait until we are allowed to decrement the token count or a signal arrives.
            while inner.token_count == 0 {
                if self.shared.state_changed.wait(&mut inner) {
                    return Err(Error::EINTR);
                }
            }

            // Consume a token.
            inner.token_count -= 1;
        }

        // Notify a possible writer waiting.
        self.shared.state_changed.notify_all();

        // Write a one-byte 1 to the reader.
        data.write_slice(&[1u8; 1])?;
        Ok(1)
    }

    fn write(&self, data: &mut UserSlicePtrReader, _offset: u64) -> KernelResult<usize> {
        {
            let mut inner = self.shared.inner.lock();

            // Wait until we are allowed to increment the token count or a signal arrives.
            while inner.token_count == MAX_TOKENS {
                if self.shared.state_changed.wait(&mut inner) {
                    return Err(Error::EINTR);
                }
            }

            // Increment the number of token so that a reader can be released.
            inner.token_count += 1;
        }

        // Notify a possible reader waiting.
        self.shared.state_changed.notify_all();
        Ok(data.len())
    }
}

struct RustFile;

impl FileOpener<()> for RustFile {
    fn open(_ctx: &()) -> KernelResult<Self::Wrapper> {
        println!("rust file was opened!");
        Ok(Box::try_new(Self)?)
    }
}

impl FileOperations for RustFile {
    type Wrapper = Box<Self>;

    kernel::declare_file_operations!();
}

struct RustExample {
    message: String,
    _chrdev: Pin<Box<chrdev::Registration<2>>>,
    _dev: Pin<Box<miscdev::Registration<Arc<SharedState>>>>,
}

impl KernelModule for RustExample {
    fn init() -> KernelResult<Self> {
        println!("Rust Example (init)");
        println!("Am I built-in? {}", !cfg!(MODULE));
        {
            let lock = THIS_MODULE.kernel_param_lock();
            println!("Parameters:");
            println!("  my_bool:    {}", my_bool.read());
            println!("  my_i32:     {}", my_i32.read(&lock));
            println!(
                "  my_str:     {}",
                core::str::from_utf8(my_str.read(&lock))?
            );
            println!("  my_usize:   {}", my_usize.read(&lock));
            println!("  my_array:   {:?}", my_array.read());
        }

        // Test mutexes.
        {
            // SAFETY: `init` is called below.
            let data = Pin::from(Box::try_new(unsafe { Mutex::new(0) })?);
            mutex_init!(data.as_ref(), "RustExample::init::data1");
            *data.lock() = 10;
            println!("Value: {}", *data.lock());

            // SAFETY: `init` is called below.
            let cv = Pin::from(Box::try_new(unsafe { CondVar::new() })?);
            condvar_init!(cv.as_ref(), "RustExample::init::cv1");
            {
                let mut guard = data.lock();
                while *guard != 10 {
                    let _ = cv.wait(&mut guard);
                }
            }
            cv.notify_one();
            cv.notify_all();
            cv.free_waiters();
        }

        // Test spinlocks.
        {
            // SAFETY: `init` is called below.
            let data = Pin::from(Box::try_new(unsafe { SpinLock::new(0) })?);
            spinlock_init!(data.as_ref(), "RustExample::init::data2");
            *data.lock() = 10;
            println!("Value: {}", *data.lock());

            // SAFETY: `init` is called below.
            let cv = Pin::from(Box::try_new(unsafe { CondVar::new() })?);
            condvar_init!(cv.as_ref(), "RustExample::init::cv2");
            {
                let mut guard = data.lock();
                while *guard != 10 {
                    let _ = cv.wait(&mut guard);
                }
            }
            cv.notify_one();
            cv.notify_all();
            cv.free_waiters();
        }

        // Including this large variable on the stack will trigger
        // stack probing on the supported archs.
        // This will verify that stack probing does not lead to
        // any errors if we need to link `__rust_probestack`.
        let x: [u64; 514] = core::hint::black_box([5; 514]);
        println!("Large array has length: {}", x.len());

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(cstr!("rust_chrdev"), 0, &THIS_MODULE)?;
        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        let state = SharedState::try_new()?;

        Ok(RustExample {
            message: "on the heap!".to_owned(),
            _dev: miscdev::Registration::new_pinned::<Token>(cstr!("rust_miscdev"), None, state)?,
            _chrdev: chrdev_reg,
        })
    }
}

impl Drop for RustExample {
    fn drop(&mut self) {
        println!("My message is {}", self.message);
        println!("Rust Example (exit)");
    }
}
