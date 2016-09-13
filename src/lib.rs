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

#![feature(asm)]
#![deny(warnings)]
#![no_std]
#![allow(unused_mut)]

#[cfg(test)]
extern crate std;

pub use platform::*;

pub mod macros;

#[cfg(all(target_os="linux", target_arch="x86"))]
#[path="platform/linux-x86/mod.rs"]
pub mod platform;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
#[path="platform/linux-x86_64/mod.rs"]
pub mod platform;

#[cfg(all(target_os="freebsd", target_arch="x86_64"))]
#[path="platform/freebsd-x86_64/mod.rs"]
pub mod platform;

#[cfg(all(target_os="linux", target_arch="arm"))]
#[path="platform/linux-armeabi/mod.rs"]
pub mod platform;

#[cfg(all(target_os="macos", target_arch="x86_64"))]
#[path="platform/macos-x86_64/mod.rs"]
pub mod platform;

#[cfg(all(target_os="linux", target_arch="aarch64"))]
#[path="platform/linux-aarch64/mod.rs"]
pub mod platform;
