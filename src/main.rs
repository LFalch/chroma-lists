extern crate chroma_lists;

use chroma_lists::ChromaList;

use std::env::args;
use std::time::Instant;

fn main() {
    let s = args().skip(1).collect::<Vec<_>>().join(" ");

    let list: ChromaList = s.trim().parse().unwrap();
    let now = Instant::now();
    let iters = list.into_iter().count();
    let duration = Instant::now() - now;

    println!("{} iterations. Took {}.{:09}s", iters, duration.as_secs(), duration.subsec_nanos());
}
