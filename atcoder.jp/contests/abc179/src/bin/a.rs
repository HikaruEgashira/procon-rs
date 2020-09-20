/**
 *    author : HikaruEgashira
 *    created:
**/
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars
    }
    let x = (x as Vec<char>)
        .iter()
        .map(|&xi| xi.to_string())
        .collect::<Vec<String>>();
    if x.last().unwrap().to_string() == "s" {
        println!("{}es", x.join(""));
    } else {
        println!("{}s", x.join(""));
    }
}
