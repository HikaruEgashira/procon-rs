/**
 *    author : HikaruEgashira
**/
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut t: Vec<String> = vec![];

    s.iter().map(|si| si.to_string()).for_each(|si| {
        if si == "R".to_string() {
            t.reverse();
        } else {
            t.push(si);
        }
    });

    let v = (0..t.len())
        .map(|_| {
            t.windows(3)
                .filter_map(|a| {
                    if a[0] != a[1] && a[1] != a[2] {
                        Some(a.first().unwrap().to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .last()
        .unwrap();

    println!("{}", v);
}
