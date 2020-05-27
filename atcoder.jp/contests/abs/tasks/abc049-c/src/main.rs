// -*- coding:utf-8-unix -*-

use proconio::input;
use regex::Regex;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        str: String
    }

    let regex = Regex::new(r"\A(dream|dreamer|erase|eraser)*\z").unwrap();
    let yes = regex.is_match(&str);

    println!("{}", if yes { "YES" } else { "NO" });
}
