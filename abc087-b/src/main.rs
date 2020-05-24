// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC087B - Coins
// https://atcoder.jp/contests/abs/tasks/abc087_b

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        sum: i32,
    }

    let mut pattern = 0;

    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if i * 500 + j * 100 + k * 50 == sum {
                    pattern += 1;
                }
            }
        }
    }

    println!("{}", pattern);
}
