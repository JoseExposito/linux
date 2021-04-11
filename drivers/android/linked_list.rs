// SPDX-License-Identifier: GPL-2.0

use alloc::{boxed::Box, sync::Arc};
use core::ptr::NonNull;

pub use crate::raw_list::{Cursor, GetLinks, Links};
use crate::{raw_list, raw_list::RawList};

// TODO: Use the one from `kernel::file_operations::PointerWrapper` instead.
pub trait Wrapper<T: ?Sized> {
    fn into_pointer(self) -> NonNull<T>;
    unsafe fn from_pointer(ptr: NonNull<T>) -> Self;
    fn as_ref(&self) -> &T;
}

impl<T: ?Sized> Wrapper<T> for Box<T> {
    fn into_pointer(self) -> NonNull<T> {
        NonNull::new(Box::into_raw(self)).unwrap()
    }

    unsafe fn from_pointer(ptr: NonNull<T>) -> Self {
        Box::from_raw(ptr.as_ptr())
    }

    fn as_ref(&self) -> &T {
        AsRef::as_ref(self)
    }
}

impl<T: ?Sized> Wrapper<T> for Arc<T> {
    fn into_pointer(self) -> NonNull<T> {
        NonNull::new(Arc::into_raw(self) as _).unwrap()
    }

    unsafe fn from_pointer(ptr: NonNull<T>) -> Self {
        Arc::from_raw(ptr.as_ptr())
    }

    fn as_ref(&self) -> &T {
        AsRef::as_ref(self)
    }
}

impl<T: ?Sized> Wrapper<T> for &T {
    fn into_pointer(self) -> NonNull<T> {
        NonNull::from(self)
    }

    unsafe fn from_pointer(ptr: NonNull<T>) -> Self {
        &*ptr.as_ptr()
    }

    fn as_ref(&self) -> &T {
        self
    }
}

pub trait GetLinksWrapped: GetLinks {
    type Wrapped: Wrapper<Self::EntryType>;
}

impl<T: ?Sized> GetLinksWrapped for Box<T>
where
    Box<T>: GetLinks,
{
    type Wrapped = Box<<Box<T> as GetLinks>::EntryType>;
}

impl<T: GetLinks + ?Sized> GetLinks for Box<T> {
    type EntryType = T::EntryType;
    fn get_links(data: &Self::EntryType) -> &Links<Self::EntryType> {
        <T as GetLinks>::get_links(data)
    }
}

impl<T: ?Sized> GetLinksWrapped for Arc<T>
where
    Arc<T>: GetLinks,
{
    type Wrapped = Arc<<Arc<T> as GetLinks>::EntryType>;
}

impl<T: GetLinks + ?Sized> GetLinks for Arc<T> {
    type EntryType = T::EntryType;
    fn get_links(data: &Self::EntryType) -> &Links<Self::EntryType> {
        <T as GetLinks>::get_links(data)
    }
}

pub struct List<G: GetLinksWrapped> {
    list: RawList<G>,
}

impl<G: GetLinksWrapped> List<G> {
    pub fn new() -> Self {
        Self {
            list: RawList::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn push_back(&mut self, data: G::Wrapped) {
        let ptr = data.into_pointer();
        if !unsafe { self.list.push_back(ptr.as_ref()) } {
            // If insertion failed, rebuild object so that it can be freed.
            unsafe { G::Wrapped::from_pointer(ptr) };
        }
    }

    pub unsafe fn insert_after(&mut self, existing: NonNull<G::EntryType>, data: G::Wrapped) {
        let ptr = data.into_pointer();
        let entry = &*existing.as_ptr();
        if !self.list.insert_after(entry, ptr.as_ref()) {
            // If insertion failed, rebuild object so that it can be freed.
            G::Wrapped::from_pointer(ptr);
        }
    }

    pub unsafe fn remove(&mut self, data: &G::Wrapped) -> Option<G::Wrapped> {
        let entry_ref = Wrapper::as_ref(data);
        if self.list.remove(entry_ref) {
            Some(G::Wrapped::from_pointer(NonNull::from(entry_ref)))
        } else {
            None
        }
    }

    pub fn pop_front(&mut self) -> Option<G::Wrapped> {
        let front = self.list.pop_front()?;
        Some(unsafe { G::Wrapped::from_pointer(front) })
    }

    pub fn cursor_front(&self) -> Cursor<'_, G> {
        self.list.cursor_front()
    }

    pub fn cursor_front_mut(&mut self) -> CursorMut<'_, G> {
        CursorMut::new(self.list.cursor_front_mut())
    }
}

impl<G: GetLinksWrapped> Drop for List<G> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct CursorMut<'a, G: GetLinksWrapped> {
    cursor: raw_list::CursorMut<'a, G>,
}

impl<'a, G: GetLinksWrapped> CursorMut<'a, G> {
    fn new(cursor: raw_list::CursorMut<'a, G>) -> Self {
        Self { cursor }
    }

    pub fn current(&mut self) -> Option<&mut G::EntryType> {
        self.cursor.current()
    }

    pub fn remove_current(&mut self) -> Option<G::Wrapped> {
        let ptr = self.cursor.remove_current()?;
        Some(unsafe { G::Wrapped::from_pointer(ptr) })
    }

    pub fn peek_next(&mut self) -> Option<&mut G::EntryType> {
        self.cursor.peek_next()
    }

    pub fn peek_prev(&mut self) -> Option<&mut G::EntryType> {
        self.cursor.peek_prev()
    }

    pub fn move_next(&mut self) {
        self.cursor.move_next();
    }
}
