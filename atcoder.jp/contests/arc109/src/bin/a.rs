/**
 *    author : HikaruEgashira
**/
use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
        y: i64,
    }

    // a -> a+-1, b -> b+-1,
    // min(y, 2 * x);

    // a -> b, a -> b-1
    // x;

    // a -> b+1
    // min(x + y, 3 * x);

    let res = if a > b {
        // 降りる
        let down = min(y, 2 * x);

        x + (a - b - 1) * down
    } else if b > a {
        // 昇る
        let up = min(y, 2 * x);
        let through = x;

        (b - a) * up + through
    } else {
        // 通る
        x
    };

    println!("{}", res);
}
