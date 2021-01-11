/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

// N M T
// Ai Bi

#[argio(output = AtCoder)]
fn main(n: i64, m: usize, t: i64, ab: [(i64, i64); m]) -> String {
    let mut x = n;
    let mut flag = true;

    ab.iter().for_each(|(a, b)| {
        x -= a;

        if x <= 0 {
            flag = false;
        }

        x += (b - a) + b;

        if x - b > n {
            x = n + b;
        }
    });

    if t >= x {
        flag = false;
    }

    if flag { "Yes" } else { "No" }.to_string()
}
