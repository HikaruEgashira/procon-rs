/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        h: f64,
        dh: [(f64, f64); n],
    }

    let res = dh
        .iter()
        .map(|&(obstacle_d, obstacle_h)| {
            obstacle_h - (((h - obstacle_h) / (d - obstacle_d)) * obstacle_d)
        })
        .fold(0.0 / 0.0, f64::max);

    if res < 0.0 {
        println!("{}", 0);
    } else {
        println!("{}", res);
    }
}
