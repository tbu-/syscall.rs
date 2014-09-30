// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Raw system calls for Rust.

#![crate_name="syscall"]
#![crate_type="lib"]

#![feature(macro_rules, asm, globs)]
#![no_std]

extern crate core;

#[cfg(test)]
extern crate std;

#[cfg(test)]
extern crate native;

pub use platform::*;

pub mod macros;

#[cfg(target_os="linux", target_arch="x86")]
#[path="platform/linux-x86/mod.rs"]
pub mod platform;

#[cfg(target_os="linux", target_arch="x86_64")]
#[path="platform/linux-x86_64/mod.rs"]
pub mod platform;

#[cfg(target_os="freebsd", target_arch="x86_64")]
#[path="platform/freebsd-x86_64/mod.rs"]
pub mod platform;
