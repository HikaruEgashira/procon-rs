// -*- coding:utf-8-unix -*-

// 12min

use proconio::input;

fn main() {
    input! {
        str: String
    }

    let parse = (str as String).parse().unwrap_or(-1);

    if parse == -1 {
        println!("{}", "error");
    } else {
        println!("{}", parse * 2)
    }
}
