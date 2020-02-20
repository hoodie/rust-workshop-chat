//! # Content
//!
//! 1. how to parse args
//! 2. read from stdin

use std::env;

fn main() {
    // the the second element from the args iterator
    if let Some(addr) = env::args().nth(1) {
        println!("thanks for passing {:#?}", addr);
    } else {
        println!("please pass a sending address")
    }
}
