#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate clap;
extern crate reqwest;

mod bench;
mod args;
mod plan;
mod engine;
mod runner;

use crate::plan::Plan;
use crate::engine::Engine;
use crate::runner::Runner;

fn main() {
	let (urls, threads, requests): (Vec<String>, usize, usize) = args::parse();

    let eng = Engine::new(urls);
    let plan = Plan::new(threads, requests);
    let runner = Runner::start(plan, &eng);

    let ((), duration) = bench::time_it(|| { runner.join() });

    let seconds =
        duration.as_secs() as f64 + (f64::from(duration.subsec_nanos()) / 1_000_000_000f64);

    println!("Finished!");
    println!();
    println!("Took {} seconds", seconds);
    println!("{} requests / second", requests as f64 / seconds);
	println!();
}
