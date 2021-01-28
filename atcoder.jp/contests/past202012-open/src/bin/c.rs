/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(mut n: usize) -> String {
    let a = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut digit: Vec<&str> = vec![];
    loop {
        if n == 0 {
            break;
        }

        let b = n % 36;
        n /= 36;
        digit.push(a.get(b..=b).unwrap());
    }

    if digit.len() == 0 {
        "0".to_string()
    } else {
        digit.iter().rev().map(|di| di.to_string()).join("")
    }
}
