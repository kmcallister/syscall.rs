// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for x86-64 Linux.

pub mod nr;

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall0(mut n: usize) -> usize {
    asm!("syscall"
         : "+{rax}"(n)
         :
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall1(mut n: usize, a1: usize) -> usize {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall2(mut n: usize, a1: usize, a2: usize) -> usize {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall3(mut n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall4(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize)
                       -> usize {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall5(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize)
                       -> usize {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4) "{r8}"(a5)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[cfg(asm)]
#[inline(always)]
pub unsafe fn syscall6(mut n: usize,
                       a1: usize,
                       a2: usize,
                       a3: usize,
                       a4: usize,
                       a5: usize,
                       a6: usize)
                       -> usize {
    asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4) "{r8}"(a5)"{r9}"(a6)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}
