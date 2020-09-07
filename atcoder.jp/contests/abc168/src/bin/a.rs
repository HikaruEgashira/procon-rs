/**
 *    author : HikaruEgashira
 *    created: 09.07.2020 19.00
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: i32) -> &str {
    let one = n % 10;
    let hon = [2, 4, 5, 7, 9];
    let pon = [0, 1, 6, 8];
    let bon = [3];
    if hon.contains(&one) {
        "hon"
    } else if pon.contains(&one) {
        "pon"
    } else if bon.contains(&one) {
        "bon"
    } else {
        ""
    }
}
