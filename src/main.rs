#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate clap;
extern crate reqwest;

mod bench;
mod args;
mod plan;
mod engine;

fn main() {
	let (urls, threads, requests): (Vec<String>, usize, usize) = args::parse();

    let eng = engine::Engine::new(urls);

    eng.run(requests);
	println!("{}", threads);
	println!("{}", requests);
}
