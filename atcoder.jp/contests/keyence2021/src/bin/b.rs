use std::usize;

/**
*    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort();

    let length: usize = *a.last().unwrap() + 1;
    let mut usable_value = k;

    let res = a
        .iter()
        .fold(vec![0; length], |mut acc, &ai| {
            acc[ai] += 1;
            acc
        })
        .iter()
        .map(|&vi| {
            if usable_value > vi {
                usable_value = vi;
            }
            usable_value
        })
        .sum::<usize>();

    println!("{}", res);
}
