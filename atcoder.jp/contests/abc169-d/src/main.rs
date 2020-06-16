// -*- coding:utf-8-unix -*-

use proconio::input;

// D - Div Game
// https://atcoder.jp/contests/abs/fasks/arc169_d

fn rec(start: i64, n: i64, count: i64) -> i64 {
    let div = (start..=n / 2).filter(|i| n % i == 0).min();
    match div {
        Some(d) => rec(d + 1, n / d, count + 1),
        None => count,
    }
}

// WIP
fn main() {
    input! {
        n: i64
    }

    let count: i64 = rec(2, n, 0);

    println!("{}", count);
}
