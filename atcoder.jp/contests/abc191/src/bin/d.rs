/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(x: f32, y: f32, r: f32) -> i64 {
    let start = (x - r).ceil() as i32;
    let end = (x + r).floor() as i32;

    (start..=end).fold(0, |acc, i| {
        let p = (r.powi(2) - (x - i as f32).powi(2)).sqrt();
        let bottom = (y - p).ceil() as i64;
        let top = (y + p).floor() as i64;
        acc + top - bottom + 1
    })
}
