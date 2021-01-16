use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, m: usize, a: [i64; n], xy: [(Usize1, Usize1); m]) -> i64 {
    let mut g = vec![vec![]; n];
    for (u, v) in xy {
        g[u].push(v);
    }

    (0..n)
        .filter(|&i| !g[i].is_empty())
        .map(|i| g[i].iter().map(|&j| solve(j, &g, &a)).max().unwrap() - a[i])
        .max()
        .unwrap()
}

#[memoise(u)]
fn solve(u: usize, g: &Vec<Vec<usize>>, a: &[i64]) -> i64 {
    g[u].iter().map(|&v| solve(v, g, a)).fold(a[u], max)
}
