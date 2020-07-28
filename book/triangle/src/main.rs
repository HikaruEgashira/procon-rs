// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let n: usize = n;
    let a: Vec<i32> = a;

    let mut res = 0;

    (0..n - 2).for_each(|i| {
        (i + 1..n - 1).for_each(|j| {
            (j + 1..n).for_each(|k| {
                if a[i] + a[j] > a[k] {
                    let len = a[i] + a[j] + a[k];
                    res = cmp::max(res, len);
                }
            })
        })
    });

    println!("{}", res);
}
