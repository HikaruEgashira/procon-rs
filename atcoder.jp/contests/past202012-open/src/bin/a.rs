/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: Chars) -> String {
    // 条件
    let is_same_value = |list: &[char]| list[0] == list[1] && list[1] == list[2];

    // 3つ毎に配列にして
    // 条件に合うか確認して
    // その中にSomeがあればそれを返す
    let data = n
        .windows(3)
        .map(|list| match is_same_value(list) {
            true => Some(list[0]),
            false => None,
        })
        .find_map(|v| v);

    match data {
        Some(v) => v.to_string(),
        None => "draw".to_string(),
    }
}
