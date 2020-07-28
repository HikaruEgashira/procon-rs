// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp;

fn main() {
    input! {
        l: i32,
        n: i32,
        x: [i32; n]
    }
    let l: i32 = l;
    let x: Vec<i32> = x;

    let min = x
        .iter()
        .map(|&xi| cmp::min(xi, (l - xi).abs()))
        .max()
        .unwrap();

    let max = x
        .iter()
        .map(|&xi| cmp::max(xi, (l - xi).abs()))
        .max()
        .unwrap();

    println!("min = {}", min);
    println!("max = {}", max);
}
