// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC081A - Placing Marbles
// https://atcoder.jp/contests/abs/tasks/abc081_a

fn main() {
    input! {
        x: String
    }
    let res = x.chars().filter(|&c| c == '1').count();
    println!("{}", res);
}
