/**
*    author : HikaruEgashira
*    created: 08.29.2020 21:19:00
**/
use proconio::input;

#[proconio::fastout]
fn my_ans_is_tle() {
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

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let (n, a): (usize, Vec<i64>) = (n, a);
    let mod_value = 1000000007;

    // 累積和を求める
    // b[i + 1] = b[i] + a[i]
    let b = a.iter().fold(vec![], |mut bi: Vec<i64>, ai| {
        bi.push(bi.last().unwrap_or(&0) + ai);
        bi
    });

    let sum = (0..n).fold(0, |acc, ni| {
        let sum = (b[n - 1] - b[ni]) % mod_value;
        (acc + a[ni] * sum) % mod_value
    });

    println!("{}", sum);
}
