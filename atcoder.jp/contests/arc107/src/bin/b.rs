/**
 *    author : HikaruEgashira
 *    created:
**/
use competitive::prelude::*;

// n = 5 k = 2
// a = 8 -> (3, 5) (4, 4) (5, 3) -> 3
// a = 7 -> (2, 5) (3, 4) (4, 3) (5, 2) -> 4
// a = 6 -> (1, 5) (2, 4) (3, 3) (4, 2) (5, 1) -> 5
// a = 5 -> (1, 4) (2, 3) (3, 2) (4, 1) -> 4
// a = 4 -> (1, 3) (2, 2) (3, 1) -> 3
// a = 3 -> (1, 2) (2, 1) -> 2
// a = 2 -> (1, 1) -> 1
fn pattern(x: i64, n: i64) -> i64 {
    let sub = x - n - 1;
    if sub > 0 {
        n - sub
    } else {
        x - 1
    }
}

#[argio(output = AtCoder)]
fn main(n: i64, k: i64) -> i64 {
    (2..=2 * n)
        .map(|ab| {
            let cd = ab - k;
            pattern(ab, n) * pattern(cd, n)
        })
        .sum()
}
