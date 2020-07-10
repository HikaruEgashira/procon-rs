// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    }

    let s: std::str::Chars = s.chars();
    let t: std::str::Chars = t.chars();

    let iter = s.zip(t);

    let res = iter.filter(|(si, ti)| si != ti).count();

    println!("{}", res);
}
