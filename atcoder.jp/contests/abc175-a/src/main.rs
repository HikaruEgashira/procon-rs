// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let rainy_days = (s as String)
        .split("S")
        .map(|rainy| rainy.len())
        .max()
        .unwrap();

    println!("{}", rainy_days);
}
