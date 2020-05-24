// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086A - Product
// https://atcoder.jp/contests/abs/tasks/abc086_a

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    let is_even = (x * y) % 2 == 0;
    println!("{}", if is_even { "Even" } else { "Odd" });
}
