/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(v: usize, t: usize, s: usize, d: usize) -> bool {
    d < v * t || v * s < d
}
