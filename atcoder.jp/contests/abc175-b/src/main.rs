// -*- coding:utf-8-unix -*-

use proconio::input;

fn a(l1: i32, l2: i32, l3: i32) -> bool {
    l1 != l2 && l2 != l3
}

fn b(l1: i32, l2: i32, l3: i32) -> bool {
    l1 + l2 > l3
}

fn main() {
    input! {
        n: usize,
        l: [i32; n],
    }
    let mut l: Vec<i32> = l;
    l.sort();

    let count = (0..n)
        .map(|n1| {
            (n1..n)
                .map(|n2| {
                    (n2..n)
                        .filter(|&n3| a(l[n1], l[n2], l[n3]) && b(l[n1], l[n2], l[n3]))
                        .count() as i32
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("{}", count);
}
