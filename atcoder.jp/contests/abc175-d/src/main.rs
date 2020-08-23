// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i32,
        p: [usize; n],
        c: [i64; n],
    }

    let res = (0..n)
        .map(|ni| {
            let mut pi = p[ni];
            let mut ci = c[ni];
            let mut sum: i64 = 0;
            (0..k).for_each(|_| {
                pi = p[pi - 1];
                ci = c[pi - 1];
                sum += ci;
            });
            sum
        })
        .max()
        .unwrap_or(0);

    println!("{}", res);
}
