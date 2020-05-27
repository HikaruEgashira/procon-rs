// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC085C - Otoshidama
// https://atcoder.jp/contests/abs/tasks/abc085_c

fn main() {
    input! {
        n: i32,
        mut sum: i32,
    }

    let mut ans: Option<(i32, i32, i32)> = None;
    'outer: for i in 0..=n {
        for j in 0..=n - i {
            let k = n - i - j;

            if i * 10000 + j * 5000 + k * 1000 == sum {
                ans = Some((i, j, k));
                break 'outer;
            }
        }
    }

    let (x, y, z) = ans.unwrap_or((-1, -1, -1));

    println!("{} {} {}", x, y, z);
}
