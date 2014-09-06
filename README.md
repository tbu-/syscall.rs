# Raw system calls for Rust

This library allows Rust code to invoke system calls directly.

It currently only supports x86-64 Linux, but more platforms can be added.

We do not intend to provide wrapper functions like `read(2)` etc. because there are many subtly different ways to define them in Rust.
