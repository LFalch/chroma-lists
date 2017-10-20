extern crate chroma_lists;

use chroma_lists::ChromaList;

use std::env::args;

fn main() {
    let s = args().skip(1).collect::<Vec<_>>().join(" ");

    let list: ChromaList = s.trim().parse().unwrap();
    let iters: Vec<_> = list.into_iter().collect();

    iters.iter().for_each(|l| println!("{}", l));
    println!("\n\n{} iterations", iters.len());
}
