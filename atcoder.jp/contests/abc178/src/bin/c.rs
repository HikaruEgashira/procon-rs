/**
 *    author : HikaruEgashira
 *    created: 09.13.2020 21.00
**/
use proconio::input;

fn mod_pow(a: i128, b: i128, mod_value: i128) -> i128 {
    (1..b).fold(a, |sum, _| (sum * a) % mod_value)
}

fn mod_mul(a: i128, b: i128, mod_value: i128) -> i128 {
    a * b % mod_value
}

fn sum(n: i128) -> i128 {
    (1..=n).sum()
}

fn main() {
    input! {
        n: i128,
    }

    let res = match n {
        1 => 0,
        2 => 2,
        n => {
            let mod_value = 10_i128.pow(9) + 7_i128;

            // 9, 1分を除く
            // (n - 2) ^ 10
            let without_zero_nine = mod_pow(10, n - 2_i128, mod_value);

            // 9, 1をいれる
            // without_zero_nine * ((without_zero_nine + 1) ^ 2) * 2
            mod_mul(
                mod_mul(without_zero_nine, 2, mod_value),
                sum(without_zero_nine + 1_i128),
                mod_value,
            )
        }
    };

    println!("{}", res);
}
