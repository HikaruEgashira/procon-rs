// -*- coding:utf-8-unix -*-

// 18min

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        n: usize,
        data: [i32; n],
    }

    (data as Vec<i32>).windows(2).for_each(|w| {
        let diff: i32 = w[1] - w[0];

        if diff > 0 {
            println!("{} {}", "up", diff)
        } else if diff < 0 {
            println!("{} {}", "down", diff.abs())
        } else {
            println!("{}", "stay")
        }
    });
}
