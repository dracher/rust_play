extern crate picalgorithms;

use picalgorithms::search;

#[test]
fn test_binary_search_01() {
    let v = vec![1, 2, 3, 4, 5, 6, 8];
    assert_eq!(0, search::binary_search(&v, 1));
}

#[test]
fn test_binary_search_02() {
    let v = vec![1, 2, 3, 4, 5, 6, 8];
    assert_eq!(2, search::binary_search(&v, 3));
}

#[test]
fn test_binary_search_03() {
    let v = vec![1, 2, 3, 4, 5, 6, 8];
    assert_eq!(6, search::binary_search(&v, 8));
}

#[test]
fn test_binary_search_04() {
    let v = vec![1, 2, 3, 4, 5, 6, 8];
    assert_eq!(-1, search::binary_search(&v, 18));
}
