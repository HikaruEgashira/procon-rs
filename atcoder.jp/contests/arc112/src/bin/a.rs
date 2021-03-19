/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn solve(l: i64, r: i64) -> i64 {
    (1..=r - l - l + 1).sum()
}

fn main() {
    input! {
        t: usize,
        lr: [(i64, i64); t],
    }

    let lr = lr as Vec<(i64, i64)>;

    lr.iter().map(|&(l, r)| solve(l, r)).for_each(|v| {
        println!("{}", v);
    });
}
