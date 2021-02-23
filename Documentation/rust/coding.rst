.. _rust_coding:

Coding
======

This document describes how to write Rust code in the kernel.


Coding style
------------

The code is automatically formatted using the ``rustfmt`` tool. This is very
good news!

- If you contribute from time to time to the kernel, you do not need to learn
  and remember one more style guide. You will also need less patch roundtrips
  to land a change.

- If you are a reviewer or a maintainer, you will not need to spend time on
  pointing out style issues anymore.

.. note:: Conventions on comments and documentation are not checked by
  ``rustfmt``. Thus we still need to take care of those: please see
   :ref:`Documentation/rust/docs.rst <rust_docs>`.

We use the tool's default settings. This means we are following the idiomatic
Rust style. For instance, we use 4 spaces for indentation rather than tabs.

Typically, you will want to instruct your editor/IDE to format while you type,
when you save or at commit time. However, if for some reason you want
to reformat the entire kernel Rust sources at some point, you may run::

	make rustfmt

To check if everything is formatted (printing a diff otherwise), e.g. if you
have configured a CI for a tree as a maintainer, you may run::

	make rustfmtcheck

Like ``clang-format`` for the rest of the kernel, ``rustfmt`` works on
individual files, and does not require a kernel configuration. Sometimes it may
even work with broken code.


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


Documentation
-------------

Please see :ref:`Documentation/rust/docs.rst <rust_docs>`.
