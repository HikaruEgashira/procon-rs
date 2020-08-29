/**
 *    author : HikaruEgashira
 *    created: 08.29.2020 21:00:00
**/
use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        d: i32,
        t: i32,
        s: i32,
    }

    println!("{}", if d <= t * s { "Yes" } else { "No" });
}
