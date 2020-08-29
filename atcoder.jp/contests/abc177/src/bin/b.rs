/**
 *    author : HikaruEgashira
 *    created: 08.29.2020 21:03:00
**/
use proconio::input;
use proconio::marker::Chars;

#[proconio::fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let (s, t): (Vec<char>, Vec<char>) = (s, t);

    let res = s
        .windows(t.len())
        .map(|a| a.iter().enumerate().filter(|&(i, &ai)| ai != t[i]).count())
        .min()
        .unwrap_or(t.len());

    println!("{}", res);
}
