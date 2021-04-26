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
        let selected_char = if max == 0 {
            ""
        } else if max == u {
            "U"
        } else if max == d {
            "D"
        } else if max == l {
            "L"
        } else {
            "R"
        };

        me = match selected_char {
            "U" => (me.0 - 1, me.1),
            "D" => (me.0 + 1, me.1),
            "L" => (me.0, me.1 - 1),
            "R" => (me.0, me.1 + 1),
            _ => (me.0, me.1),
        };

        print!("{}", selected_char);
    });

    println!("");
}
