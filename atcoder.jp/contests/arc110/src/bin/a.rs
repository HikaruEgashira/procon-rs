/**
*    author : HikaruEgashira
**/
use num::integer::lcm;
use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut base = 2;

    (3..=n).for_each(|ni| {
        base = lcm(base, ni);
    });

    println!("{}", base + 1);
}
