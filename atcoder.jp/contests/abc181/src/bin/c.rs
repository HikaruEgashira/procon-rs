/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, mut xy: [(i32, i32); n]) -> String {
    let res = (0..n - 2)
        .map(|a| {
            (a + 1..n - 1)
                .map(|b| {
                    (b + 1..n)
                        .filter(|&c| {
                            let xy1 = xy[a];
                            let xy2 = xy[b];
                            let xy3 = xy[c];

                            let dx1 = xy1.0 - xy2.0;
                            let dy1 = xy1.1 - xy2.1;
                            let dx2 = xy2.0 - xy3.0;
                            let dy2 = xy2.1 - xy3.1;

                            dx2 * dy1 == dx1 * dy2
                        })
                        .count()
                })
                .sum()
        })
        .collect::<Vec<usize>>();

    if res.iter().sum::<usize>() > 0 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}
