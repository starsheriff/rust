// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Basic test for free regions in the NLL code. This test ought to
// report an error due to a reborrowing constraint. Right now, we get
// a variety of errors from the older, AST-based machinery (notably
// borrowck), and then we get the NLL error at the end.

// compile-flags:-Znll

fn foo<'a, 'b>(x: &'a u32, y: &'b u32) -> &'b u32 {
    &*x //~ ERROR free region `'a` does not outlive `'b`
    //~^ ERROR `*x` does not live long enough
    //~| WARN not reporting region error due to -Znll
}

fn main() { }
