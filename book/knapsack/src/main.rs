// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp;

struct DP {
    dp: Vec<Vec<i32>>,
    n: usize,              // 個数
    wv: Vec<(usize, i32)>, // (重さ, 価値)
    weight: usize,         // 重さの総和
}

impl DP {
    #[allow(dead_code)]
    fn rec(&mut self, i: usize, j: usize) -> i32 {
        if self.dp[i][j] > 0 {
            return self.dp[i][j];
        }

        let res;
        if i == self.n {
            res = 0;
        } else if j < self.wv[i].0 {
            res = self.rec(i + 1, j);
        } else {
            res = cmp::max(
                self.rec(i + 1, j),
                self.rec(i + 1, j - self.wv[i].0) + self.wv[i].1,
            )
        }

        // memorize
        self.dp[i][j] = res;

        res
    }

    #[allow(dead_code)]
    fn solve(&mut self) -> i32 {
        self.rec(0, self.weight)
    }
}

fn main() {
    input! {
        n: usize,
        weight: usize,
        wv: [(usize, i32); n],  // Vec<(i32, i32)>
    }

    // initialize
    let dp = (0..100)
        .map(|_| (0..100).map(|_| 0).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let res = DP { dp, n, weight, wv }.solve();

    println!("{}", res);
}
