/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        a: i64,
    }

    println!("{}", if a % 2 == 0 { "White" } else { "Black" });
}
