/**
*    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    let mut prev_max = a[0] * b[0];
    let mut prev_max_a = a[0];
    (0..(n as usize)).for_each(|ni| {
        if prev_max_a < a[ni] {
            prev_max_a = a[ni];
        }

        let cur = prev_max_a * b[ni];

        if prev_max < cur {
            prev_max = cur;
        }

        println!("{}", prev_max);
    });
}
