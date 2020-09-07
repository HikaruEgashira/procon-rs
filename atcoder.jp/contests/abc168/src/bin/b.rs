/**
 *    author : HikaruEgashira
 *    created: 09.07.2020 19.03
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(k: usize, s: String) -> String {
    if s.len() <= k {
        s
    } else {
        format!("{}...", s.get(0..k).unwrap().to_string())
    }
}
