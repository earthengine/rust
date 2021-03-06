// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;

#[repr(transparent)]
struct Foo(u32);

const TRANSMUTED_U32: u32 = unsafe { mem::transmute(Foo(3)) };
//~^ ERROR The use of std::mem::transmute() is gated in constants (see issue #53605)

fn main() {}
