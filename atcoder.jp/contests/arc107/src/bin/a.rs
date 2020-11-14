/**
 *    author : HikaruEgashira
 *    created:
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(a: i128, b: i128, c: i128) -> i128 {
    let mod_value = 998244353;

    // (1..=a).for_each(|ai| {
    //     (1..=b).for_each(|bi| {
    //         (1..=c).for_each(|ci| {
    //             sum += ai * bi * ci;
    //         })
    //     })
    // });

    // (1..=a).for_each(|ai| {
    //     (1..=b).for_each(|bi| {
    //         sum += ai * bi * (ai + bi);
    //     })
    // });

    let ai = a * (a + 1) / 2 % mod_value;
    let bi = b * (b + 1) / 2 % mod_value;
    let ci = c * (c + 1) / 2 % mod_value;

    ((ai * bi) % mod_value) * ci % mod_value
}
