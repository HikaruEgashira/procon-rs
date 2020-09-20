/**
 *    author : HikaruEgashira
 *    created:
**/
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n]
    }
    let d: Vec<(usize, usize)> = d;
    let bin = d
        .iter()
        .map(|&(i, j)| if i == j { 1 } else { 0 })
        .collect::<Vec<i32>>();
    let mut flag = false;
    let mut count = 0;
    bin.iter().for_each(|&bini| {
        if bini == 1 {
            count += 1;
            if count == 3 {
                flag = true;
            }
        } else {
            count = 0;
        }
    });
    println!("{}", if flag { "Yes" } else { "No" });
}
