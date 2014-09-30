# Raw system calls for Rust

[![Build Status](https://travis-ci.org/kmcallister/syscall.rs.svg?branch=master)](https://travis-ci.org/kmcallister/syscall.rs)

This library allows Rust code to invoke system calls directly.

See the [list of supported platforms](https://github.com/kmcallister/syscall.rs/tree/master/src/platform).  Additions are very welcome!

We do not intend to provide wrapper functions like `read(2)` etc. because there are many subtly different ways to define them in Rust.
