// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC169B - Multiplication 2
// https://atcoder.jp/contests/abc169/tasks/abc169_b

fn main() {
    input! {
        n: usize,
        mut datas: [i64; n],
    }

    datas.sort();

    let o: Option<i64> = datas
        .iter()
        .map(|&v| v as i64)
        .fold(Some(1), |sum, i| match sum {
            Some(num) => num.checked_mul(i),
            None => None,
        });

    let res = match o {
        Some(num) => {
            if num > 10i64.pow(18) {
                -1
            } else {
                num
            }
        }
        None => -1,
    };

    println!("{}", res);
}
