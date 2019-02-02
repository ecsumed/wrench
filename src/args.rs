use clap::{Arg, App, SubCommand};
use super::stats::ChartSize;

pub fn parse() -> (Vec<String>, usize, usize, ChartSize) {
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
                          .arg(Arg::with_name("chart-size")
                               .long("chart-size")
                               .takes_value(true)
                               .possible_values(&["none", "n", "small", "s", "medium", "m", "large", "l"])
                               .help("The size of the chart to render"))
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

    let chart_size = match matches.value_of("chart-size").unwrap_or("medium") {
        "none" | "n" => ChartSize::None,
        "small" | "s" => ChartSize::Small,
        "medium" | "m" => ChartSize::Medium,
        "large" | "l" => ChartSize::Large,
        _ => unreachable!(),
    };

    (urls, threads, requests, chart_size)
}
