/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, x: i64, a: [i64; n]) -> Vec<i64> {
    a.iter()
        .filter(|&&ai| ai != x)
        .map(|&v| v)
        .collect::<Vec<i64>>()
}
