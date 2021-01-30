/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, s: i64, d: i64, xy: [(i64, i64); n]) -> String {
    let data = xy.iter().find(|&&(x, y)| x < s && y > d).is_some();
    if data { "Yes" } else { "No" }.to_string()
}
