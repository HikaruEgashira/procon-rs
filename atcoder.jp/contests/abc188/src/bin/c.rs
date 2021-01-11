use std::usize;

/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let len = (2 as usize).pow(n as u32) as usize;

    input! {
        a: [i32; len],
    }

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

    let res = if left_max.1 > right_max.1 {
        right_max.0
    } else {
        left_max.0
    };

    println!("{}", res);
}
