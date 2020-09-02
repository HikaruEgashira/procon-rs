/**
 *    author : HikaruEgashira
 *    created:
**/
use competitive::prelude::*;

fn calc(x: i32, k: i32, a: &Vec<i32>) -> i32 {
    let sum = a
        .iter()
        .map(|&ai| if ai % x == 0 { ai / x - 1 } else { ai / x })
        .sum::<i32>();

    if sum > k {
        1
    } else if sum < k {
        2
    } else {
        0
    }
}

#[argio(output = AtCoder)]
fn main(n: usize, k: i32, a: [i32; n]) -> i32 {
    let mut valid_arr = vec![];

    // 二分探索
    let mut min = *a.iter().min().unwrap_or(&0);
    let mut max = *a.iter().max().unwrap_or(&(10 ^ 9));
    let mut center = (min + max) / 2;
    while min != max {
        let calc = calc(center, k, &a);
        match calc {
            0 => {
                return calc;
            }
            1 => {
                min = center;
                valid_arr.push(center);
            }
            2 => {
                max = center;
            }
            _ => panic!("unreachable"),
        }
        center = (min + max) / 2;
    }

    *valid_arr.iter().min().unwrap_or(a.iter().max().unwrap())
}
