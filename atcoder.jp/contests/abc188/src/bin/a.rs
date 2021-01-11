/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(a: i32, b: i32) -> String {
    let diff = (a - b).abs();
    if diff >= 3 {
        "No".to_string()
    } else {
        "Yes".to_string()
    }
}
