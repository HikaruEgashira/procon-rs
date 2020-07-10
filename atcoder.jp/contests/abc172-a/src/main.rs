// -*- coding:utf-8-unix -*-

use proconio::input;
use num::pow;

fn main() {
    input! {
        n: i32,
    }
    let res = n + pow(n, 2) + pow(n, 3);
    println!("{}", res);
}
