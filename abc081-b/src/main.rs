// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC081B - Shift only
// https://atcoder.jp/contests/abs/tasks/abc081_b

fn main() {
    input! {
        n: usize,
        mut plan: [i32; n],
    }

    let mut operation_count = 0;

    while plan.iter().all(|x| *x % 2 == 0) {
        for x in plan.iter_mut() {
            *x /= 2;
        }
    
        operation_count += 1;
    }
    
    println!("{}", operation_count);
}
