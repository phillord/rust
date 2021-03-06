// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(const_fn, thread_local)]

#[thread_local]
static A: u32 = 1;

static B: u32 = A;
//~^ ERROR thread-local statics cannot be accessed at compile-time

static C: &u32 = &A;
//~^ ERROR thread-local statics cannot be accessed at compile-time

const D: u32 = A;
//~^ ERROR thread-local statics cannot be accessed at compile-time

const E: &u32 = &A;
//~^ ERROR thread-local statics cannot be accessed at compile-time

const fn f() -> u32 {
    A
    //~^ ERROR thread-local statics cannot be accessed at compile-time
}

fn main() {}
