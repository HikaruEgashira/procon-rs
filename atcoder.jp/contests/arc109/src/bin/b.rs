/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn solve(a: i64, b: i64, n: i64) -> i64 {
    let target = (a + b) / 2;
    let sum = (target + 1) * (target + 2) / 2 - 1;

    if a + 1 == b {
        return target;
    }

    if sum > n {
        solve(a, target, n)
    } else if sum < n {
        solve(target, b, n)
    } else {
        target
    }
}

fn main() {
    input! {
        n: i64,
    }

    let res = n - solve(0, n, n);

    println!("{}", res);
}
