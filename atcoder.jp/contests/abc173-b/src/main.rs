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

    let mut judge_result = Judge {
        ac: 0,
        wa: 0,
        tle: 0,
        re: 0,
    };
    judge.iter().for_each(|j| {
        let ju: &str = j;
        match ju {
            "AC" => judge_result.ac = judge_result.ac + 1,
            "WA" => judge_result.wa = judge_result.wa + 1,
            "TLE" => judge_result.tle = judge_result.tle + 1,
            "RE" => judge_result.re = judge_result.re + 1,
            _ => {}
        }
    });

    println!("AC x {}", judge_result.ac);
    println!("WA x {}", judge_result.wa);
    println!("TLE x {}", judge_result.tle);
    println!("RE x {}", judge_result.re);
}
