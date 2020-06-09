// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC169C - Multiplication 3
// https://atcoder.jp/contests/abc169/tasks/abc169_c

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let res = (a * b) as i64;

    println!("{}", res);
}
