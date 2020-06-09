// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC083B - Some Sums
// https://atcoder.jp/contests/abs/tasks/abc083_b

// ★
fn char2num(c: char) -> i32 {
    (c as i32) - 48
}

// ★
fn sum_of_digits(n: i32) -> i32 {
    n.to_string().chars().map(char2num).sum::<i32>()
}

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    // RangeInclusiveは実質iter
    let sum_result = (1..=n)
        .filter(|&i| {
            let sum_of_digits = sum_of_digits(i);
            a <= sum_of_digits && sum_of_digits <= b
        })
        //
        // ↓をするときれいなi32のiterに
        // .collect::<Vec<i32>>()
        // .iter()
        //
        // ↓== .fold(0, |sum, i| sum + i);
        .sum::<i32>();

    println!("{}", sum_result);
}
