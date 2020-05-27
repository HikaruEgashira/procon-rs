// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC088B - Card Game for Two
// https://atcoder.jp/contests/abs/tasks/abc088_b

fn main() {
    input! {
        n: usize, // 1 <= N <= 100
        mut v: [i32; n], // 1 <= a <= 100 []
    }

    // 降順ソート
    v.sort_by(|x, y| y.cmp(x));

    let mut alice = 0;
    let mut bob = 0;
    for (i, &x) in v.iter().enumerate() {
        if i % 2 == 0 {
            alice += x;
        } else {
            bob += x;
        }
    }

    println!("{}", alice - bob);
}
