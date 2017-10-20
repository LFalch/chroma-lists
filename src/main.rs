extern crate clap;
extern crate chroma_lists;

use std::fs::File;
use std::io::Write;
use std::time::Instant;

use clap::{App, Arg};

use chroma_lists::ChromaList;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                          .version(env!("CARGO_PKG_VERSION"))
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .about(env!("CARGO_PKG_DESCRIPTION"))
                          .arg(Arg::with_name("SEQUENCE")
                               .help("Sequence to run through algorithm")
                               .multiple(true)
                               .required(true)
                               .takes_value(true))
                          .arg(Arg::with_name("steps")
                               .short("f")
                               .long("steps-file")
                               .takes_value(true)
                               .help("File to output all the intermediate steps into"))
                          .get_matches();
    let mut output_file = matches.value_of("steps").map(|p| File::create(p).unwrap());

    let s = matches.values_of("SEQUENCE").unwrap().collect::<Vec<_>>().join(" ");

    let list: ChromaList = s.trim().parse().unwrap();
    let now = Instant::now();
    let iters = list.into_iter().inspect(|s| if let Some(ref mut o) = output_file {write!(o, "{}", s).unwrap()}).count();
    let duration = Instant::now() - now;

    println!("{} iterations. Took {}.{:09}s", iters, duration.as_secs(), duration.subsec_nanos());
}
