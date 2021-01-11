/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(a: i64, b: i64, c: i64, d: i64) -> i64 {
    *vec![a, b, c, d].iter().min().unwrap()
}
