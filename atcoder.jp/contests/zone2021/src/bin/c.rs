/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        n: usize,
        ability: [(i64, i64, i64, i64, i64); n],
    }

    // 全列挙
    let v: Vec<(usize, usize, usize)> = (0..n)
        .map(|a| (a..n).map(move |b| (b..n).map(move |c| (a, b, c))))
        .flatten()
        .flatten()
        .collect();

    // vそれぞれの総合力
    let data: Vec<i64> = v
        .iter()
        .map(|&(a, b, c)| {
            *vec![ability[a], ability[b], ability[c]]
                .iter()
                .fold(
                    vec![vec![], vec![], vec![], vec![], vec![]],
                    |mut acc, &a| {
                        acc[0].push(a.0);
                        acc[1].push(a.1);
                        acc[2].push(a.2);
                        acc[3].push(a.3);
                        acc[4].push(a.4);
                        acc
                    },
                )
                .iter()
                .map(|v| v.iter().max().unwrap())
                .min()
                .unwrap()
        })
        .collect();

    println!("{}", data.iter().max().unwrap());
}
