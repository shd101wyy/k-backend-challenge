#![feature(box_patterns)]
extern crate immutable_map;

use std::time::{SystemTime};
// use std::rc::Rc;
// use std::collections::HashMap;

mod opt;
mod direct;


fn main() {
    let timer = SystemTime::now();
    let c = direct::start(direct::sum_pgm(1000000));
    // let c = direct::start(direct::sum_pgm(10_000_000));
    // let c = start(sum_pgm(10_000_000));
    direct::run(c);

    let elapsed = timer.elapsed().unwrap();
    let ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

    println!("execution time  {}ms", ms);
}
