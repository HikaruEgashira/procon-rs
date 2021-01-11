/**
 *    author : HikaruEgashira
**/
use competitive::{number::fact, prelude::*};

#[argio(output = AtCoder)]
fn main(n: usize) -> i128 {
    (n - 11..n).fold(1, |a, b| a * b as i128) / fact::<i128>(11)
}
