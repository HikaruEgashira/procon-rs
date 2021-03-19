/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        b: i64,
        c: i64,
    }
    let (b, c) = (b as i64, c as i64);

    let x = c / 2;
    let y = c % 2;

    let data = if y == 1 {
        // 奇数
        let (min, max) = if b > 0 {
            (-b - x, b + x - 1)
        } else {
            (b - x, -b + x)
        };

        max - min
    } else {
        // 偶数
        let (min, max) = if b > 0 {
            (-b - x - 1, b + x - 1)
        } else {
            (b - x, -b + x - 1)
        };

        max - min
    };

    println!("{}", data);
}
