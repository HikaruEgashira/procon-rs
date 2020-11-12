/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, ab: [(i64, i64); n]) -> i64 {
    // 2 3 4 5 6 -> 8 * 2 + 4
    // 2 3 4 5 6 7 -> 9 * 3
    ab.iter()
        .map(|&(a, b)| {
            if (b - a + 1) % 2 == 0 {
                (a + b) * ((b - a + 1) / 2)
            } else {
                (a + b) * ((b - a + 1) / 2) + (a + b) / 2
            }
        })
        .sum()
}
