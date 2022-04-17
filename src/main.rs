extern crate rokoko;

use rokoko::prelude::*;

const C: ivec3 = ivec3::new(1, (2, 3), ());

fn main() {
    println!("{:?}", C)
}
