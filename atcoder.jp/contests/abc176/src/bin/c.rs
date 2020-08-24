// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let sum = (a as Vec<i64>)
        .iter()
        // sum -- 台の合計
        // max -- 今見ているイテレータ以前の人の身長の最大値
        // ai  -- 今見ているイテレータの身長
        .fold((0, 0), |(sum, max), &ai| {
            if max > ai {
                // 台の追加
                (sum + max - ai, max)
            } else {
                // 身長の最大値の更新
                (sum, ai)
            }
        })
        .0;

    println!("{}", sum);
}
