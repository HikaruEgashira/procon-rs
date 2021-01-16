use std::{cmp::Ordering, usize};

/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, a: [i64; 2_usize.pow(n as _)]) -> usize {
    let len = a.len();

    // (index, rate)
    let mut left_max = (0, 0);
    let mut right_max = (0, 0);

    (0..len).for_each(|leni| {
        if leni < len / 2 {
            // left
            if left_max.1 < a[leni] {
                left_max = (leni + 1, a[leni]);
            }
        } else {
            // right
            if right_max.1 < a[leni] {
                right_max = (leni + 1, a[leni]);
            }
        }
    });

    match (left_max.1).cmp(&right_max.1) {
        Ordering::Less => left_max.0,
        Ordering::Greater => right_max.0,
        _ => panic!("Cannot solve"),
    }
}
