/**
*    author : HikaruEgashira
*    created: 08.29.2020 21:19:00
**/
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let a: Vec<i64> = a;

    let mod_value = 1000000007;

    let sum = (0..n - 1)
        .map(|na| a[na] * (na + 1..n).map(|nb| a[nb]).sum::<i64>())
        .sum::<i64>()
        % mod_value;

    println!("{}", sum);
}
