#![allow(dead_code, bare_trait_objects)]

#[macro_use]
extern crate amplify_derive;

#[derive(Getters)]
struct One {
    a: Vec<u8>,
    b: bool,
}

fn main() {}