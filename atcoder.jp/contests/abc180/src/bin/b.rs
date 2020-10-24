/**
 *    author : HikaruEgashira
 *    created:
**/
use competitive::prelude::*;

fn manhattan_distance(x: &Vec<i64>) -> i64 {
    x.iter().map(|&num| num.abs()).sum()
}

fn euclidean_distance(x: &Vec<i64>) -> f64 {
    x.iter()
        .map(|&num| (num.abs() as f64).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn chebyshev_distance(x: &Vec<i64>) -> i64 {
    x.iter().map(|&num| num.abs()).max().unwrap()
}

#[argio(output = AtCoder)]
fn main(n: usize, x: [i64; n]) -> String {
    format!(
        "{}\n{}\n{}",
        manhattan_distance(&x),
        euclidean_distance(&x),
        chebyshev_distance(&x)
    )
}
