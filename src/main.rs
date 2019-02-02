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
mod message;
mod collector;
mod content_length;
mod chart;
mod stats;

use crate::plan::Plan;
use crate::engine::Engine;
use crate::runner::Runner;
use crate::stats::{ChartSize, Fact, Summary};

fn main() {
	let (urls, threads, requests, chart_size): (Vec<String>, usize, usize, ChartSize) = args::parse();

    let eng = Engine::new(urls);
    let plan = Plan::new(threads, requests);

    let (collector, recv_handle) = collector::start::<Fact>(plan.clone());
    let runner = Runner::start(plan, &eng, &collector);

    println!("\nBeginning requests");
    let ((), duration) = bench::time_it(|| { runner.join() });

     let facts = recv_handle.join().expect("Receiving thread to finish");

    let seconds =
        duration.as_secs() as f64 + (f64::from(duration.subsec_nanos()) / 1_000_000_000f64);

    println!("Finished!\n");
    println!("Took {} seconds", seconds);
    println!("{} requests / second", requests as f64 / seconds);
	println!();
    println!(
        "{}",
        Summary::from_facts(&facts).with_chart_size(chart_size)
	);
}
