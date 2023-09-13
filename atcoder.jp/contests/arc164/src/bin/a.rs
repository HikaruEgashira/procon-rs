/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        t: i64,
        case: [(i64, i64); t]
    }

    for (n, k) in case {
        assert!(1<=t && t<=10_i64.pow(5));
        assert!(1<=k && k<=n && n<=10_i64.pow(18));
        println!("{}", yes_or_no(solve(n, k)));
    }
}

// exist M | N = 3^m_1 + 3^m_2 + ... + 3^m_k
fn solve(n: i64, k: i64) -> bool {
    let mut n = n;
    let mut ans = 0;
    while n > 0 {
        ans += n % 3;
        n /= 3;
    }
    ans <= k && (k - ans) % 2 == 0
}

fn yes_or_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}
