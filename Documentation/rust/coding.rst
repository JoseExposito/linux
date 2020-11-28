.. _rust_coding:

Coding
======

This document describes how to write Rust code in the kernel.


Coding style
------------

For the moment, we are following the idiomatic Rust style. For instance,
we use 4 spaces for indentation rather than tabs.


Abstractions vs. bindings
-------------------------

We don't have abstractions for all the kernel internal APIs and concepts,
but we would like to expand coverage as time goes on. Unless there is
a good reason not to, always use the abstractions instead of calling
the C bindings directly.

If you are writing some code that requires a call to an internal kernel API
or concept that isn't abstracted yet, consider providing an (ideally safe)
abstraction for everyone to use.


Conditional compilation
-----------------------

Rust code has access to conditional compilation based on the kernel
configuration:

.. code-block:: rust

	#[cfg(CONFIG_X)]      // `CONFIG_X` is enabled (`y` or `m`)
	#[cfg(CONFIG_X="y")]  // `CONFIG_X` is enabled as a built-in (`y`)
	#[cfg(CONFIG_X="m")]  // `CONFIG_X` is enabled as a module   (`m`)
	#[cfg(not(CONFIG_X))] // `CONFIG_X` is disabled

