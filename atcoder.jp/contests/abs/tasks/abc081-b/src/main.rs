// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC081B - Shift only
// https://atcoder.jp/contests/abs/tasks/abc081_b

fn pow_num(number: i32, i: i32) -> i32 {
    if number % 2 == 0 {
        pow_num(number / 2, i + 1)
    } else {
        i
    }
}

// O(^2)
fn main() {
    input! {
        n: usize,
        nums: [i32; n],
    }

    let count = nums
        .iter()
        // 2で何回割り切れるかの配列に
        .map(|num| pow_num(*num, 0))
        // ↑で作成した配列の最小値を求める
        .fold(std::i32::MAX, |m, v| v.min(m));

    println!("{}", count);
}
