// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        t: i32,
    }

    let alpha = if n % x == 0 { 0 } else { 1 };
    let res = (n / x + alpha) * t;

    println!("{}", res);
}
