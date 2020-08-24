// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn char2num(c: char) -> i32 {
    (c as i32) - 48
}

fn main() {
    input! {
        n: Chars
    }

    let sum_of_digit = (n as Vec<char>).iter().map(|&ci| char2num(ci)).sum::<i32>();
    let res = sum_of_digit % 9 == 0;

    println!("{}", if res { "Yes" } else { "No" });
}
