use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        if re.find(&line).is_some() {
            println!("{}", line);
        }
    }
}

fn main() {
    let args = App::new("grep-light")
        .version("0.1")
        .about("Works almost like normal grep :DSearch for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("Pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("input")
                .help("File to search in")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let input = args.value_of("input").unwrap_or("-");
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let reader: Box<dyn BufRead> = if input == "-" {
        let stdin = io::stdin();
        Box::new(stdin.lock())
    } else {
        let f = File::open(input).unwrap();
        Box::new(BufReader::new(f))
    };

    process_lines(reader, re);
}
