/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64,
        z: f64,
    }

    let per_one_a = y / x;
    let float_res = (y / x * z).round();
    let per_one_b = float_res / z;

    let res = if per_one_a > per_one_b {
        float_res as i64
    } else {
        float_res as i64 - 1
    };

    println!("{}", res);
}
