#![feature(box_patterns)]
extern crate immutable_map;
extern crate slab;
extern crate typed_arena;

use std::time::{SystemTime};

mod opt; // I couldn't compile, so I give up it.  
mod direct;
mod vec;
mod arena;
// mod mutable;


fn main() {
    let n = 10_000_000;
    // let n = 10000000;

    
    // ===== direct ========================
    let timer = SystemTime::now();
    let c = direct::start(direct::sum_pgm(n)); // <= modify this line for different `n`
    direct::run(c);

    let elapsed = timer.elapsed().unwrap();
    let ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

    println!("direct: execution time  {}ms", ms);

    // ===== optimized ========================

    let timer = SystemTime::now();
    let c = opt::start(opt::sum_pgm(n)); // <= modify this line for different `n`
    opt::run(c);

    let elapsed = timer.elapsed().unwrap();
    let ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

    println!("optimized: execution time  {}ms", ms);

    // ===== vec ========================
    let timer = SystemTime::now();
    let c = vec::start(vec::sum_pgm(n)); // <= modify this line for different `n`
    vec::run(c);

    let elapsed = timer.elapsed().unwrap();
    let ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

    println!("vec: execution time  {}ms", ms);

    // ===== arena ========================
    let timer = SystemTime::now();
    arena::run_pgm(n); // <= modify this line for different `n`

    let elapsed = timer.elapsed().unwrap();
    let ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

    println!("arena: execution time  {}ms", ms);
}
