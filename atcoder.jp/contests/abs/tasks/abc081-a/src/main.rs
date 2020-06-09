// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC081A - Placing Marbles
// https://atcoder.jp/contests/abs/tasks/abc081_a

fn char2num(c: char) -> i32 {
    (c as i32) - 48
}

fn main() {
    input! {
        x: String
    }

    let res = x.chars().map(char2num).filter(|&c| c == 1).count();

    println!("{}", res);
}
