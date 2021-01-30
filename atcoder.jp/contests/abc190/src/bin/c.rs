/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(n: usize, m: usize, ab: [(usize, usize); m], k: usize, cd: [(usize, usize); k]) -> usize {
    let mut ab_data = ab
        .iter()
        .map(|&(a, b)| vec![a, b])
        .flatten()
        .collect::<Vec<usize>>()
        .iter()
        .fold(vec![0; n + 1], |mut acc, &v| {
            acc[v] += 1;
            acc
        });
    let ab_max_pos = (0..n)
        .map(|_| {
            let max_value = ab_data.iter().position_max().unwrap();
            ab_data[max_value] = 0;
            max_value
        })
        .collect::<Vec<usize>>();

    let mut cd_data = cd
        .iter()
        .map(|&(a, b)| vec![a, b])
        .flatten()
        .collect::<Vec<usize>>()
        .iter()
        .fold(vec![0; n + 1], |mut acc, &v| {
            acc[v] += 1;
            acc
        });
    let mut cd_max_pos = (0..n)
        .map(|_| {
            let max_value = cd_data.iter().position_max().unwrap();
            cd_data[max_value] = 0;
            max_value
        })
        .collect::<Vec<usize>>();
    cd_max_pos.reverse();

    let data = cd
        .iter()
        .map(|&(c, d)| {
            let c_pos = cd_max_pos
                .iter()
                .find_position(|&&a| a == c)
                .unwrap_or((0, &n))
                .0;
            let d_pos = cd_max_pos
                .iter()
                .find_position(|&&a| a == d)
                .unwrap_or((0, &n))
                .0;
            if c_pos > d_pos {
                cd_max_pos.remove(c_pos);
                c
            } else {
                cd_max_pos.remove(d_pos);
                d
            }
        })
        .collect::<Vec<usize>>();
    dbg!(data);
    ab_max_pos[0]
}
