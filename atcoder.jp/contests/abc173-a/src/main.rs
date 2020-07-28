// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: i32,
    }

    let diff = a % 1000;

    let res = match diff {
        0 => 0,
        pos => 1000 - pos,
    };

    println!("{}", res);
}
