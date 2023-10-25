use crate::garden::vegetables::Asparagus;
use rand::Rng;
use std::collections::HashMap;

use std::cmp::Ordering;

use std::io::{self, Write};

// wild card
use std::collections::*;

pub mod garden;

// use std::fmt;
// use std::io::Result as IoResult;

// fn function1() -> fmt::Result {
//     // --snip--
// }
//
// fn function2() -> IoResult<()> {
//     // --snip--
// }

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);

    let mut map = HashMap::new();
    map.insert(1, 2);
}
