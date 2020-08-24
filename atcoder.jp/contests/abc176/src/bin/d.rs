// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        size: (usize, usize),
        c: (usize, usize),
        d: (usize, usize),
        s: [Chars; size.0]
    }

    // 始点
    let c: (usize, usize) = c;

    // 終点
    let d: (usize, usize) = d;

    // 迷路
    let s: Vec<Vec<char>> = s;

    let is_maze =
        |x: i8, y: i8| -> bool { x < (size.0 as i8) && x >= 0 && y < (size.1 as i8) && y >= 0 };
    let is_path = |x: usize, y: usize, s: &Vec<Vec<char>>| -> bool { s[x][y] == '#' };

    // 動的計画法みたいな箱
    let mut dy = vec![vec![-1; size.0 + 1]; size.1 + 1];

    // (cnt, x, y)
    let mut queue: Vec<(i32, usize, usize)> = vec![];
    queue.push((0, c.0, c.1));

    while let Some(q) = queue.pop() {
        let (cnt, x, y) = q;

        if (x, y) == d {
            println!("{}", cnt);
            return;
        }

        // すでに通ってたら探索しない
        if dy[x][y] != -1 && dy[x][y] <= cnt {
            continue;
        }
        dy[x][y] = cnt;

        // 移動 A
        [-1, 1].iter().for_each(|&xi| {
            [-1, 1].iter().for_each(|&yi| {
                let new_x = (q.0 as i8) + xi;
                let new_y = (q.1 as i8) + yi;

                if is_maze(new_x, new_y) && is_path(new_x as usize, new_y as usize, &s) {
                    queue.push((0, (new_x as usize), (new_y as usize)));
                }
            });
        });

        // 移動 B
        [-2, 2].iter().for_each(|&xi| {
            [-2, 2].iter().for_each(|&yi| {
                let new_x = (q.0 as i8) + xi;
                let new_y = (q.1 as i8) + yi;

                if is_maze(new_x, new_y) && is_path(new_x as usize, new_y as usize, &s) {
                    queue.push((1, (new_x as usize), (new_y as usize)));
                }
            });
        });
    }

    println!("{}", -1);
}
