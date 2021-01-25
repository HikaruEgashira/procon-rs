/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, a: [i64; n]) -> i64 {
    a.iter()
        .enumerate()
        .map(|(i, &ai)| {
            let mut sum = 0;
            let mut val = ai;

            (i..n).for_each(|ii| {
                if a[ii] >= val {
                    sum += val;
                } else {
                    val = 0;
                }
            });

            val = ai;
            (0..i).rev().for_each(|ii| {
                if a[ii] >= val {
                    sum += val;
                } else {
                    val = 0;
                }
            });

            sum
        })
        .max()
        .unwrap_or(0)
}
