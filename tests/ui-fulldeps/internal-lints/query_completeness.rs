//@ compile-flags: -Z unstable-options
// #[cfg(bootstrap)]: We can stop ignoring next beta bump; afterward this ALWAYS should run.
//@ ignore-stage1 (requires matching sysroot built with in-tree compiler)
#![feature(rustc_private)]
#![deny(rustc::untracked_query_information)]

extern crate rustc_data_structures;

use rustc_data_structures::steal::Steal;

fn use_steal(x: Steal<()>) {
    let _ = x.is_stolen();
    //~^ ERROR `is_stolen` accesses information that is not tracked by the query system
}

fn main() {}
