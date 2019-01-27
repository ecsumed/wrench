#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate clap;

mod bench;
mod args;
mod plan;

fn main() {
	let (urls, threads, requests): (Vec<String>, usize, usize) = args::parse();

	println!("{}", threads);
	println!("{}", requests);
}
