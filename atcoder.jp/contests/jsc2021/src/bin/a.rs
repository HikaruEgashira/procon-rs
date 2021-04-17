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
    let float_res = y / x * z;
    let per_one_b = float_res.round() / z;

    let res = if per_one_a > per_one_b {
        float_res.round() as i64
    } else {
        float_res.round() as i64 - 1
    };

    println!("{}", res);
}
