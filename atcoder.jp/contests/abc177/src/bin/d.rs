/**
*    author : HikaruEgashira
*    created: 08.29.2020 21:36:00
**/
use proconio::input;

// fn push_vec(group: Vec<Vec<usize>>, a: usize, b: usize) -> Vec<Vec<usize>> {
//     let mut out: Vec<Vec<usize>> = group.clone();
//     out[a].push(b);
//     out[b].push(a);
//     out
// }

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [(usize, usize); m],
    }

    let a: Vec<(usize, usize)> = a;

    let mut group: Vec<Vec<usize>> = vec![vec![]; n];

    for (ai, bi) in a.clone() {
        for aii in group[ai - 1].clone() {
            if !group[aii - 1].contains(&bi) {
                group[aii - 1].push(bi - 1);
            }
        }
        for bii in group[bi - 1].clone() {
            if !group[bii - 1].contains(&ai) {
                group[bii - 1].push(bi);
            }
        }

        if !group[ai - 1].contains(&bi) {
            group[ai - 1].push(bi);
        }
        if !group[bi - 1].contains(&ai) {
            group[bi - 1].push(ai);
        }
    }

    let res = group.iter().map(|g| g.iter().count()).max().unwrap();

    println!("{}", res);
}
