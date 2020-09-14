/**
 *    author : HikaruEgashira
 *    created: 09.13.2020 21.00
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(a: i64, b: i64, c: i64, d: i64) -> i64 {
    let res = vec![a * c, a * d, b * c, b * d];
    *res.iter().max().unwrap()
}
