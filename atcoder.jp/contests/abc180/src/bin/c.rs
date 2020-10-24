/**
 *    author : HikaruEgashira
 *    created:
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(x: i64) -> String {
    let mut res = vec![];
    let to = (x as f64).sqrt().trunc() as i64;
    (1..=to).for_each(|i| {
        if x % i == 0 {
            res.push(i);
            if i != x / i {
                res.push(x / i)
            }
        }
    });
    res.sort();
    res.iter().join("\n")
}
