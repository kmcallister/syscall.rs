// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for arm Linux.

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret : usize;
    llvm_asm!("swi $$0" : "={r0}"(ret)
                   : "{r7}"(n)
                   : "memory" "cc"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let ret : usize;
    llvm_asm!("swi $$0" : "={r0}"(ret)
                   : "{r7}"(n), "{r0}"(a1)
                   : "memory" "cc"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret : usize;
    llvm_asm!("swi $$0" : "={r0}"(ret)
                   : "{r7}"(n), "{r0}"(a1), "{r1}"(a2)
                   : "memory" "cc"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret : usize;
    llvm_asm!("swi $$0" : "={r0}"(ret)
                   : "{r7}"(n), "{r0}"(a1), "{r1}"(a2), "{r2}"(a3)
                   : "memory" "cc"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize) -> usize {
    let ret : usize;
    llvm_asm!("swi $$0" : "={r0}"(ret)
                   : "{r7}"(n), "{r0}"(a1), "{r1}"(a2), "{r2}"(a3),
                     "{r3}"(a4)
                   : "memory" "cc"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize, a5: usize) -> usize {
    let ret : usize;
    llvm_asm!("swi $$0" : "={r0}"(ret)
                   : "{r7}"(n), "{r0}"(a1), "{r1}"(a2), "{r2}"(a3),
                     "{r3}"(a4), "{r4}"(a5)
                   : "memory" "cc"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall6(n: usize, a1: usize, a2: usize, a3: usize,
                                a4: usize, a5: usize, a6: usize) -> usize {
    let ret : usize;
    llvm_asm!("swi $$0" : "={r0}"(ret)
                   : "{r7}"(n), "{r0}"(a1), "{r1}"(a2), "{r2}"(a3),
                     "{r3}"(a4), "{r4}"(a5), "{r5}"(a6)
                   : "memory" "cc"
                   : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall7(n: usize, a1: usize, a2: usize, a3: usize,
                            a4: usize, a5: usize, a6: usize,
                            a7: usize) -> usize {
    let ret : usize;
    llvm_asm!("swi $$0" : "={r0}"(ret)
                   : "{r7}"(n), "{r0}"(a1), "{r1}"(a2), "{r2}"(a3),
                     "{r3}"(a4), "{r4}"(a5), "{r5}"(a6), "{r6}"(a7)
                   : "memory" "cc"
                   : "volatile");
    ret
}
