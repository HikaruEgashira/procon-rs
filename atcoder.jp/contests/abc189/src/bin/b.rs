/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, x: f32, vp: [(f32, f32); n]) -> i32 {
    let mut sum = 0.0;
    let data = vp
        .iter()
        .map(|&(v, p)| {
            sum += v * p * 0.01;
            sum
        })
        .find_position(|&a| a > x);

    if let Some(vv) = data {
        (vv.0 as i32) + 1
    } else {
        -1
    }
}
