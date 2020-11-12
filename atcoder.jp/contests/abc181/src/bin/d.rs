// -*- coding:utf-8-unix -*-

use permutohedron::LexicalPermutation;
use proconio::input;
use proconio::marker::Chars;

fn char2num(c: char) -> i32 {
    (c as i32) - 48
}

fn main() {
    input! {
        n: Chars
    }

    let mut res = false;

    let digit = (n as Vec<char>)
        .iter()
        .map(|&ci| char2num(ci))
        .collect::<Vec<i32>>();

    let len = digit.len();
    match len {
        1 => res = 8 == digit[0],
        2 => res = digit[1] * 10 + digit[0] % 8 == 0 || digit[0] * 10 + digit[1] % 8 == 0,
        _ => (0..len - 2).for_each(|a| {
            (a + 1..len - 1).for_each(|b| {
                (b + 1..len).for_each(|c| {
                    let mut values = [a, b, c];
                    loop {
                        if (digit[values[0]] * 100 + digit[values[1]] * 10 + digit[values[2]]) % 8
                            == 0
                        {
                            res = true;
                            break;
                        }
                        if !values.next_permutation() {
                            break;
                        }
                    }
                })
            })
        }),
    }

    println!("{}", if res { "Yes" } else { "No" });
}
