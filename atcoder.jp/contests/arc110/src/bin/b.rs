/**
*    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
    }

    let max = 10_000_000_000;

    let a = (n + 2) / 3;
    let b = if (n + 2) % 3 == 0 { a } else { a + 1 };
    let v = "110".repeat(b).split(&t as &str).count();

    let res = match &t as &str {
        "1" => max * 2,
        "0" => max,
        _ => {
            if v == 1 {
                0
            } else {
                match n % 3 {
                    0 => max - (n as i64 / 3) + 1,
                    _ => max - (n as i64 / 3),
                }
            }
        }
    };

    println!("{}", res);
}
