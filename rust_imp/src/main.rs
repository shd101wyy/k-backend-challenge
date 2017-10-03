#![feature(box_patterns)]
extern crate immutable_map;

use std::time::{SystemTime};

// mod opt; // I couldn't compile, so I give up it.  
mod direct;


fn main() {
    let timer = SystemTime::now();
    let c = direct::start(direct::sum_pgm(10_000_000)); // <= modify this line for different `n`
    direct::run(c);

    let elapsed = timer.elapsed().unwrap();
    let ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

    println!("execution time  {}ms", ms);
}
