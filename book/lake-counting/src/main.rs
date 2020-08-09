// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

#[allow(unused_variables)]
fn main() {
    input! {
        n: usize,
        m: usize,
        lake: [Chars; n]
    }
    let lake: Vec<Vec<char>> = lake;
    lake.iter().for_each(|ll| {
        ll.iter().for_each(|l| {
            print!("{}", l);
        })
    })
}
