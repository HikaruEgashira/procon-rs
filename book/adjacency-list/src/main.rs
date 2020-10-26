use competitive::prelude::*;

#[derive(Clone, Debug)]
struct Vertex {
    edge: Vec<usize>,
}

#[argio(output = AtCoder)]
fn main(n: usize, l: usize, v: [(usize, usize); l]) -> usize {
    let mut graph: Vec<Vertex> = vec![Vertex { edge: vec![] }; n];

    v.iter().for_each(|&(a, b)| graph[a].edge.push(b));

    dbg!(graph);
    // graph = [
    //     Vertex {
    //         edge: [
    //             1,
    //             2,
    //         ],
    //     },
    //     Vertex {
    //         edge: [
    //             2,
    //         ],
    //     },
    //     Vertex {
    //         edge: [],
    //     },
    // ]

    n
}

/*
3 3
0 1
0 2
1 2
*/
