extern crate rust_landscape;
extern crate regex;
extern crate clap;

use regex::Regex;
use clap::{Arg,App};
use rust_landscape::Landscape;


pub fn main() {
    let matches = App::new("Rust landscape rain filling calculator")
        .version("0.1.0")
        .author("Antonio Estévez <a.estevez.blanco@gmail.com>")
        .arg(Arg::with_name("landscape")
            .short("l")
            .long("landscape")
            .value_name("\"1, 2, 3, 4\"")
            .help("Array representing the landscape")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("rain_hours")
            .short("h")
            .long("hours-of-rain")
            .value_name("POSITIVE_NUMBER")
            .help("Hours of rain")
            .required(true)
            .takes_value(true))
        .get_matches();
    let landscape_match = matches.value_of("landscape").unwrap();
    let hours_match = matches.value_of("rain_hours").unwrap();

    let heights = parse_landscape(landscape_match);
    let hours = parse_hours(hours_match);

    let mut landscape = Landscape::new(&heights);
    landscape.rain(hours);

    let segments = landscape.get_segments();
    for segment in segments.iter() {
        println!(
            "Segment {} with height {} has {} units of rain",
            segment.index,
            segment.height,
            segment.rain,
        )
    }
}

fn parse_landscape(landscape: &str) -> Vec<usize> {
    let mut size: Vec<usize> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();
    for cap in re.captures_iter(landscape) {
        let num = &cap[0].parse::<usize>().unwrap();
        size.push(*num);
    }
    size
}

fn parse_hours(hours: &str) -> usize {
    hours.parse::<usize>().unwrap()
}
