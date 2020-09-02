/**
 *    author : HikaruEgashira
 *    created: 09.01.2020
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(k: usize) -> i32 {
    let mut a = vec![];

    // 漸化式

    // i == 1
    let a_1 = 7 % k;
    a.push(a_1);

    // i >= 2
    (2..=k).for_each(|_| {
        let ai_1 = a.last().unwrap();
        let ai = (ai_1 * 10 + 7) % k;
        a.push(ai);
    });

    match a.iter().enumerate().find(|&(_, &ki)| ki == 0) {
        Some((i, _)) => {i as i32 + 1}, // インデックスを個数に
        None => {-1}, // 見つからなかった
    }
}
