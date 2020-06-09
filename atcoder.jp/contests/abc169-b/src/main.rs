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

    let calc: Option<i64> = datas
        .iter()
        // ↓これ消したい
        .map(|&v| v as i64)
        .fold(Some(1), |sum, i| match sum {
            Some(num) => num.checked_mul(i),
            None => None,
        });

    let max = 10_i64.pow(18);
    let none_value = -1;
    let res = match calc {
        Some(num) => {
            if num > max {
                none_value
            } else {
                num
            }
        }
        None => none_value,
    };

    println!("{}", res);
}
