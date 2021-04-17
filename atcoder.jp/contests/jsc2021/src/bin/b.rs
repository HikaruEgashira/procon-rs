/**
 *    author : HikaruEgashira
**/
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; m],
    }

    let a_hash: HashSet<i32> = a.into_iter().collect();
    let b_hash: HashSet<i32> = b.into_iter().collect();

    let res_a: Vec<String> = a_hash
        .difference(&b_hash)
        .map(|&h| format!("{}", h))
        .collect();
    let res_b: Vec<String> = b_hash
        .difference(&a_hash)
        .map(|&h| format!("{}", h))
        .collect();

    let res = [res_a.join(" "), res_b.join(" ")].join(" ");
    println!("{}", res);
}
