// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        n: usize,
        k: i32,
        p: [i32; n],
        c: [i32; n],
    }

    println!("{} {} {} {}", n, k, p[0], c[0]);
}
