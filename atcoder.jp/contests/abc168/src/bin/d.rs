/**
 *    author : HikaruEgashira
 *    created:
**/
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let ab: Vec<(usize, usize)> = ab;

    let mut graph: Vec<Vec<usize>> = vec![vec![0; n]; n];
    ab.iter().for_each(|&(a, b)| {
        graph[a][b] = 1;
        graph[b][a] = 1;
    });

    let shortest_path = vec![0; n]
        .iter()
        .map(|&i| {
            let mut x: Vec<(usize, usize)> = vec![(1, 0)];
            while let Some(target) = x.pop() {
                if target.1 == i {
                    continue;
                }
                graph[target.0]
                    .iter()
                    .enumerate()
                    .find_map(|(a, &i)| if a == 1 { Some(i) } else { None })
                    .iter()
                    .for_each(|a| x.push((*a, target.1)));
            }
            x
        }) // つかれた
        .collect::<Vec<Option<usize>>>();

    println!("{}", n);
}
