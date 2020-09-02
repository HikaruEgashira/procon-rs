/**
 *    author : HikaruEgashira
 *    created: 09.01.2020
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, d: i64, xy: [(i64, i64); n]) -> usize {
    xy.iter().filter(|&(x, y)| {
        x.pow(2) + y.pow(2) <= d.pow(2)
    }).count()
}
