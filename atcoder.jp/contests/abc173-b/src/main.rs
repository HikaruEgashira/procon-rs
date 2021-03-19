// -*- coding:utf-8-unix -*-

use proconio::input;

struct Judge {
    ac: i32,
    wa: i32,
    tle: i32,
    re: i32,
}

fn main() {
    input! {
        n: usize,
        judge: [String; n],
    }
    let judge: Vec<String> = judge;

    let judge_result = judge.iter().fold(
        Judge {
            ac: 0,
            wa: 0,
            tle: 0,
            re: 0,
        },
        |mut acc, s| {
            match s as &str {
                "AC" => acc.ac = acc.ac + 1,
                "WA" => acc.wa = acc.wa + 1,
                "TLE" => acc.tle = acc.tle + 1,
                "RE" => acc.re = acc.re + 1,
                _ => {}
            };
            acc
        },
    );

    println!("AC x {}", judge_result.ac);
    println!("WA x {}", judge_result.wa);
    println!("TLE x {}", judge_result.tle);
    println!("RE x {}", judge_result.re);
}
