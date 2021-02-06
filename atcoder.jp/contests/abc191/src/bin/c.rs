/**
 *    author : HikaruEgashira
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(h: usize, _: usize, s: [Chars; h]) -> usize {
    // let mut pass_top = false;
    // let mut pass_buttom = false;
    // let mut data = 0;

    // s.iter().for_each(|w| {
    //     let vec_w = w.clone() as Vec<char>;
    //     if vec_w.iter().find(|&&c| c == '#').is_some() {
    //         if !pass_top {
    //             pass_top = true;
    //             data += vec_w.iter().filter(|&&c| c == '#').count();
    //         }
    //     } else {
    //         if !pass_buttom {
    //             pass_buttom = false;
    //         }
    //     };
    //     vec_w.iter().for_each(|&si| {
    //         si;
    //     });
    // });
    4
}

// .....
// .###.
// .###.
// .###.
// .....

// 0
// 1
// 1
// 1
// 0

// 0
// 1
// 1
// 1
// 0

// .....
// ..##.
// .###.
// .###.
// .....

// 0
// 2
// 1
// 1
// 0

// 0
// 2
// 1
// 1
// 0
