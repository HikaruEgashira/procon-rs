/**
*    author : HikaruEgashira
*    created: 08.29.2020 21:36:00
**/
use proconio::input;

#[derive(Debug, Clone)]
enum UF {
    Root(usize),
    Child(usize),
}

#[derive(Debug)]
struct UnionFind {
    table: Vec<UF>,
}
impl UnionFind {
    // 初期化
    fn new(n: usize) -> UnionFind {
        UnionFind {
            table: vec![UF::Root(1); n],
        }
    }
    // 親
    fn root(&mut self, x: usize) -> usize {
        match self.table[x] {
            UF::Root(_) => x,
            UF::Child(parent) => {
                let root = self.root(parent);
                self.table[x] = UF::Child(root);
                root
            }
        }
    }
    // 同じグループか判定
    #[allow(dead_code)]
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    // Root(size) ← sizeの値を出力
    fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        match self.table[r] {
            UF::Root(size) => size,
            UF::Child(_) => 0,
        }
    }
    // 併合
    fn merge(&mut self, x: usize, y: usize) {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x != root_y {
            let size_x = self.size(root_x);
            let size_y = self.size(root_y);
            let (i, j) = if size_x > size_y {
                (root_x, root_y)
            } else {
                (root_y, root_x)
            };
            self.table[i] = UF::Root(size_x + size_y);
            self.table[j] = UF::Child(i);
        }
    }
}

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [(usize, usize); m],
    }

    let a: Vec<(usize, usize)> = a;

    // UnionFind木の作成
    let mut uf = UnionFind::new(n);
    for mi in 0..m {
        let (x, y) = a[mi];
        uf.merge(x - 1, y - 1);
    }

    // 木の大きさの最大値を求める
    let ans = (0..n).map(|i| uf.size(i)).max().unwrap();

    println!("{}", ans);
}

#[allow(dead_code)]
fn my_ans_wa() {
    input! {
        n: usize,
        m: usize,
        a: [(usize, usize); m],
    }

    let a: Vec<(usize, usize)> = a;

    let mut group: Vec<Vec<usize>> = vec![vec![]; n];

    for (ai, bi) in a.clone() {
        for aii in group[ai - 1].clone() {
            if !group[aii - 1].contains(&bi) {
                group[aii - 1].push(bi - 1);
            }
        }
        for bii in group[bi - 1].clone() {
            if !group[bii - 1].contains(&ai) {
                group[bii - 1].push(bi);
            }
        }

        if !group[ai - 1].contains(&bi) {
            group[ai - 1].push(bi);
        }
        if !group[bi - 1].contains(&ai) {
            group[bi - 1].push(ai);
        }
    }

    let res = group.iter().map(|g| g.iter().count()).max().unwrap();

    println!("{}", res);
}
