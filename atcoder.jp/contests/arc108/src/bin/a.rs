/**
 *    author : HikaruEgashira
**/
use proconio::input;

// binary search
fn solve(start: i64, end: i64, s: i64, p: i64) -> String {
    let key = (start + end + 1) / 2;
    let n = key;
    let m = s - n;
    let nm = n.checked_mul(m);

    if p == 0 {
        "Yes".to_string()
    } else if nm < Some(p) {
        if start == n {
            "No".to_string()
        } else {
            solve(n, end, s, p)
        }
    } else if nm > Some(p) || nm.is_none() {
        if end == n {
            "No".to_string()
        } else {
            solve(start, n, s, p)
        }
    } else {
        "Yes".to_string()
    }
}

fn main() {
    input! {
        s: i64,
        p: i64
    }

    let res = solve(0, (s + 1) / 2, s, p);

    println!("{}", res);
}
