extern crate picalgorithms;

use picalgorithms::search;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let ret = search::binary_search(&v, 6);

    println!("{}", ret);
}
