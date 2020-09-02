/**
 *    author : HikaruEgashira
 *    created:
**/
use competitive::prelude::*;

// WRWWRW RRRWR
// RRRRRR WWWWW
// 101101 11101

#[argio(output = AtCoder)]
fn main(_: usize, c: Chars) -> usize {
    let c: Vec<char> = c;
    let r_count = c.iter().filter(|&&ci| ci == 'R').count();
    (0..r_count).filter(|&ri| c[ri] == 'W').count()
}
