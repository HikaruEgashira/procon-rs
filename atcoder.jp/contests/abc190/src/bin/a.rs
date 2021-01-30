use competitive::prelude::*;
/**
*    author : HikaruEgashira
**/
use std::cmp::Ordering;

#[argio(output = AtCoder)]
fn main(a: usize, b: usize, c: usize) -> String {
    let a_user = "Takahashi";
    let b_user = "Aoki";

    let win = match a.cmp(&b) {
        Ordering::Equal => {
            if c == 0 {
                b_user
            } else {
                a_user
            }
        }
        Ordering::Greater => a_user,

        Ordering::Less => b_user,
    };

    win.to_string()
}
