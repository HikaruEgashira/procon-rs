// -*- coding:utf-8-unix -*-

use proconio::input;

struct Prop {
    n: usize,
    a: Vec<i32>,
    k: i32,
}

// depthと現在の状態(sum)で現depthより下を調べる
fn depth_first_search(depth: usize, sum: i32, prop: &Prop) -> bool {
    if depth == prop.n {
        return sum == prop.k;
    }

    depth_first_search(depth + 1, sum, prop)
        || depth_first_search(depth + 1, sum + prop.a[depth], prop)
}

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        k: i32
    }

    let solve = depth_first_search(0, 0, &Prop { n, a, k });

    println!("{}", if solve { "Yes" } else { "No" });
}
