/**
 *    author : HikaruEgashira
**/
use proconio::input;
use proconio::marker::Chars;

// binary search
fn solve(s: Vec<char>) -> usize {
    let mut stack = vec![];
    let mut len = 0;

    s.iter().for_each(|&si| match si {
        'f' => stack.push(si),
        'o' => stack.push(si),
        'x' => {
            if let Some(a) = stack.pop() {
                if let Some(b) = stack.pop() {
                    if a == 'o' && b == 'f' {
                    } else {
                        len += 3 + stack.len();
                        stack.clear();
                    }
                } else {
                    stack.push(a)
                }
            } else {
                len += 1;
            }
        }
        _ => {
            len += 1 + stack.len();
            stack.clear();
        }
    });

    len + stack.len()
}

fn main() {
    input! {
        _: i64,
        s: Chars,
    }

    let res = solve(s);

    println!("{}", res);
}
