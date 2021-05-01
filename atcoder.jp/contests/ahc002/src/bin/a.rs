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
        // フラグの更新
        dp = (0..50)
            .map(|x| {
                (0..50)
                    .map(|y| t[me.0][me.1] == t[x][y] || dp[x][y])
                    .collect()
            })
            .collect();

        let pos_u = (if me.0 == 0 { 0 } else { me.0 - 1 }, me.1);
        let pos_d = (me.0 + 1, me.1);
        let pos_l = (me.0, if me.1 == 0 { 0 } else { me.1 - 1 });
        let pos_r = (me.0, me.1 + 1);

        let u = p.get(pos_u.0).map(|d| d.get(pos_u.1)).flatten();
        let d = p.get(pos_d.0).map(|d| d.get(pos_d.1)).flatten();
        let l = p.get(pos_l.0).map(|d| d.get(pos_l.1)).flatten();
        let r = p.get(pos_r.0).map(|d| d.get(pos_r.1)).flatten();

        let max = *vec![u, d, l, r].iter().max().unwrap();
        let selected_char = match max {
            Some(v) => {
                if v == u.unwrap_or(&10000) {
                    "U"
                } else if v == d.unwrap_or(&10000) {
                    "D"
                } else if v == l.unwrap_or(&10000) {
                    "L"
                } else {
                    "R"
                }
            }
            None => "",
        };

        me = match selected_char {
            "U" => pos_u,
            "D" => pos_d,
            "L" => pos_l,
            "R" => pos_r,
            _ => me,
        };

        print!("{}", selected_char);
    });

    println!("");
}
