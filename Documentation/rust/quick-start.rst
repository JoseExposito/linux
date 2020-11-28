.. _rust_quick_start:

Quick Start
===========

This document describes how to get started with Rust kernel development.


Requirements
------------

rustc and cargo
***************

A recent nightly Rust toolchain (with, at least, ``rustc`` and ``cargo``)
is required, e.g. ``nightly-2020-08-27``. In the future, this restriction
will be lifted.

If you are using ``rustup``, run::

    rustup toolchain install nightly

Otherwise, fetch a standalone installer from:

    https://www.rust-lang.org


rustc sources
*************

The sources for the compiler are required to be available because the standard
library (``core`` and ``alloc``) is cross-compiled.

If you are using ``rustup``, run::

    rustup component add rust-src

Otherwise, if you used a standalone installer, you can clone the Rust compiler
sources and create a symlink to them in the installation folder of
your nightly toolchain::

    git clone https://github.com/rust-lang/rust
    ln -s rust .../rust-nightly/lib/rustlib/src


bindgen
*******

The bindings to the C side of the kernel are generated at build time using
``bindgen``. Currently we assume the latest version available, but that
may change in the future.

Install it via::

    cargo install bindgen


rustfmt
*******

Optionally, if you install ``rustfmt``, then you will get the generated
C bindings automatically formatted. It is also useful to have the tool
to format your own code, too.

If you are using ``rustup``, its ``default`` profile already installs it,
so you should be good to go. If you are using another one, you can also
install the component::

    rustup component add rustfmt

The standalone installers also come with ``rustfmt``.


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

