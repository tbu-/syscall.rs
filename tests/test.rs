// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(phase)]

#[phase(plugin, link)]
extern crate syscall;

// getpid() is POSIX but that doesn't guarantee it's a system call.
#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
#[test]
fn getpid() {
    unsafe {
        assert!(0 < syscall::syscall0(syscall::nr::GETPID));
    }
}

#[cfg(target_os="linux")]
#[cfg(target_os="freebsd")]
#[test]
fn getpid_macro() {
    unsafe {
        assert!(0 < syscall!(GETPID));
    }
}
