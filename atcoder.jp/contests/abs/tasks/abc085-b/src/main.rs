// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC085B - Kagami Mochi
// https://atcoder.jp/contests/abs/tasks/abc085_b

fn main() {
    input! {
        n: usize,
        mut list: [i32; n],  // Vec<(i32, i32, i32)>
    }

    // ★重複削除（ソートされる）
    list.sort();
    list.dedup();

    let res = list.len();
    println!("{}", res);
}
