/**
 *    author : HikaruEgashira
 *    created:
**/
use proconio::input;
use proconio::marker::Chars;

#[proconio::fastout]
fn main() {
    input! {
        h: usize, w: usize, k: usize,
        c: [Chars; h],
    }

    let (h, w, k, c): (usize, usize, usize, Vec<Vec<char>>) = (h, w, k, c);

    // 選ばない
    let no_choose = c.iter().map(|ci| ci.iter().map(|&cii| cii == '#')).count() == k;
    let count = c
        .iter()
        .enumerate()
        .map(move |(y, ci)| {
            ci.iter()
                .enumerate()
                .filter(move |&(x, &cii)| x != w && y != h && cii == '#')
                .count()
        })
        .filter(|&ii| ii == k)
        .count();

    println!("{}", if no_choose { count + 1 } else { count });
}
