/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, a: [i64; n], b: [i64; n]) -> String {
    let res = (0..n).fold(0, |acc, ni| acc + a[ni] * b[ni]);
    if res != 0 {
        "No".to_string()
    } else {
        "Yes".to_string()
    }
}
