// -*- coding:utf-8-unix -*-

use proconio::input;

fn nth_sum(vec: &Vec<i64>, n: usize) -> i64 {
    (0..n).fold(0, |acc, x| acc + vec[x])
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a: [i64; n],
        b: [i64; m],
    };

    let k: i64 = k;
    let a: Vec<i64> = a;
    let b: Vec<i64> = b;

    // O(n + m)
    let a_sum = (1..=n).map(|i| nth_sum(&a, i)).collect::<Vec<i64>>();
    let b_sum = (1..=m).map(|i| nth_sum(&b, i)).collect::<Vec<i64>>();

    // O(n * m)
    let count = (0..n)
        .map(|ai: usize| {
            let diff = k - a_sum[ai];
            let b_count = b_sum.iter().filter(|&&c| c <= diff).count() + 1;

            if diff < 0 {
                0
            } else {
                ai + b_count
            }
        })
        .max()
        .unwrap_or(0);

    println!("{}", count);
}
