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
    n.to_string().chars().map(|c| char2num(c)).sum::<i32>()
}

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let sum_result = (1..=n)
        .filter(|ni| {
            let sum_of_digits = sum_of_digits(*ni);
            a <= sum_of_digits && sum_of_digits <= b
        })
        .sum::<i32>();

    println!("{}", sum_result);
}
