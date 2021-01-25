use competitive::prelude::Chars;
/**
*    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        a: Chars,
    }

    let res = if a[0] == a[1] && a[1] == a[2] {
        "Won"
    } else {
        "Lost"
    };

    println!("{}", res);
}
