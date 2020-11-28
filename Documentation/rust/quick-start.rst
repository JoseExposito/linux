.. _rust_quick_start:

Quick Start
===========

This document describes how to get started with Rust kernel development.


Requirements
------------

A recent nightly Rust toolchain (at least ``rustc``, ``cargo`` and ``rustfmt``)
is required, e.g. ``nightly-2020-08-27``. In the future, this restriction
will be lifted. If you are using ``rustup``, run::

    rustup toolchain install nightly

Otherwise, fetch a standalone installer from:

    https://www.rust-lang.org

The sources for the compiler are required to be available. If you are using
``rustup``, run::

    rustup component add rust-src

Otherwise, if you used a standalone installer, you can clone the Rust compiler
sources and create a symlink to them in the installation folder of
your nightly toolchain::

    git clone https://github.com/rust-lang/rust
    ln -s rust .../rust-nightly/lib/rustlib/src


Testing a simple driver
-----------------------

If the kernel configuration system is able to find ``rustc`` and ``cargo``,
it will enable Rust support (``CONFIG_HAS_RUST``). In turn, this will make
visible the rest of options that depend on Rust.

A simple driver you can compile to test things out is at
``drivers/char/rust_example`` (``CONFIG_RUST_EXAMPLE``). Enable it and compile
the kernel with:

    make LLVM=1

TODO: drop LLVM=1, allowing to mix GCC with LLVM


Avoiding the network
--------------------

You can prefetch the dependencies that ``cargo`` will download by running::

    cargo fetch

in the kernel sources root. After this step, a network connection is
not needed anymore.

