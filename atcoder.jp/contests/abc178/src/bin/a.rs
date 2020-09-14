/**
 *    author : HikaruEgashira
 *    created: 09.13.2020 21.00
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(x: i32) -> i32 {
    match x {
        0 => 1,
        1 => 0,
        _ => 0,
    }
}
