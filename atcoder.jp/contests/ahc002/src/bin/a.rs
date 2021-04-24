/**
 *    author : HikaruEgashira
**/
use proconio::input;

fn main() {
    input! {
        s: (usize, usize),
        t: [[usize; 50]; 50],
        p: [[usize; 50]; 50],
    }

    let mut me = s;
    let mut dp = vec![vec![false; 50]; 50];

    (0..200).for_each(|_| {
        dp = (0..50)
            .map(|x| {
                (0..50)
                    .map(|y| t[me.0][me.1] == t[x][y] || dp[x][y])
                    .collect()
            })
            .collect();

        let u = if me.0 == 0 {
            0
        } else {
            if dp[me.0 - 1][me.1] {
                0
            } else {
                p[me.0 - 1][me.1]
            }
        };

        let d = if me.0 == 49 {
            0
        } else {
            if dp[me.0 + 1][me.1] {
                0
            } else {
                p[me.0 + 1][me.1]
            }
        };

        let l = if me.1 == 0 {
            0
        } else {
            if dp[me.0][me.1 - 1] {
                0
            } else {
                p[me.0][me.1 - 1]
            }
        };

        let r = if me.1 == 49 {
            0
        } else {
            if dp[me.0][me.1 + 1] {
                0
            } else {
                p[me.0][me.1 + 1]
            }
        };

        let max = **[&u, &d, &l, &r].iter().max().unwrap();
        let c = if max == 0 {
            ""
        } else if max == u {
            me = (me.0 - 1, me.1);
            "U"
        } else if max == d {
            me = (me.0 + 1, me.1);
            "D"
        } else if max == l {
            me = (me.0, me.1 - 1);
            "L"
        } else {
            me = (me.0, me.1 + 1);
            "R"
        };

        print!("{}", c);
    });

    println!("");
}
