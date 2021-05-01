/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let a = s.split("ZONe").count() - 1;

    println!("{}", a);
}
