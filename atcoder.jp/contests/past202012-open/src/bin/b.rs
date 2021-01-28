/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(_: usize, s: Chars) -> String {
    let mut arr = s as Vec<char>;

    // 逆から検査したら削除する手間が省ける
    arr.reverse();
    arr = arr.iter().fold(vec![], |mut acc: Vec<char>, c| {
        if !acc.contains(c) {
            acc.push(*c);
        }
        acc
    });
    arr.reverse();

    arr.iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .concat()
}
