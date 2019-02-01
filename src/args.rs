use clap::{Arg, App, SubCommand};

pub fn parse() -> (Vec<String>, usize, usize) {
    let matches = App::new("Wrench benchmarker.")
                          .version("1.0")
                          .author("Fahad S. <ecsumed@yahoo.com>")
                          .about("Simple http benchmarker.")
                          .arg(Arg::with_name("concurrent")
                               .short("c")
                               .value_name("concurrent")
                               .takes_value(true)
                               .default_value("1")
                               .help("Number of concurrent requests (threads)."))
                          .arg(Arg::with_name("requests")
                               .short("n")
                               .required(true)
                               .default_value("1")
                               .help("Number of total requests.")
                               .takes_value(true))
                          .arg(Arg::with_name("URLs")
                               .multiple(true)
                               .required(true)
                               .help("URLs are round robin'ed."))
                          .get_matches();

    let urls: Vec<String> = matches
        .values_of("URLs")
        .expect("URLs are required.")
        .map(|v| v.to_string())
        .collect();

    let threads = matches
        .value_of("concurrent")
        .unwrap_or("1")
        .parse::<usize>()
        .expect("Expected valid number for threads");

    let requests = matches
        .value_of("requests")
        .unwrap_or("1000")
        .parse::<usize>()
        .expect("Expected valid number for number of requests");

    (urls, threads, requests)
}
