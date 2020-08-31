/**
 *    author : HikaruEgashira
 *    created: 08.01.2020
**/
use proconio::input;

// @エラトステネスの篩.rs
fn factors(max: usize) -> Vec<Vec<usize>> {
    let mut factors = vec![vec![]; max];
    (2..factors.len()).for_each(|i| {
        if factors[i].is_empty() {
            (i..factors.len()).step_by(i).for_each(|j| {
                factors[j].push(i);
            });
        }
    });
    factors
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a: Vec<usize> = a;

    const MAX_A: usize = 1_000_001;
    let factors = factors(MAX_A);

    // a           0   1   2    3
    // [6, 9] -> [[], [], [6], [6, 9], ..]
    let mut prime_pos = vec![vec![]; MAX_A];
    a.iter().for_each(|&ai| {
        factors[ai].iter().for_each(|&p| {
            prime_pos[p].push(ai);
        })
    });

    // `if a.iter()prime_pos[fi].len()`が2以上なら公約数があるってことになる
    let pairwise = a
        .iter()
        .all(|&ai| factors[ai].iter().all(|&fi| prime_pos[fi].len() == 1));
    // 因数分解表にa全てが含まれる素数がない
    let setwise = !prime_pos.contains(&a);

    let res = if pairwise {
        "pairwise coprime"
    } else if setwise {
        "setwise coprime"
    } else {
        "not coprime"
    };

    println!("{}", res);
}
