// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: i32,
        k: i32,
        d: i32,
    }
    let x: i32 = x;
    let k: i32 = k;
    let d: i32 = d;

    let a = x / d;
    let b = x % d;

    if a > k {
        println!("{}", (b - (k - a) % 2 * d).abs());
    } else {
        println!("{}", (x - d * k).abs());
    }
}
