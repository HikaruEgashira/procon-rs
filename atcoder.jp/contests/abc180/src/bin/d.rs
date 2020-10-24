/**
 *    author : HikaruEgashira
 *    created:
**/
use competitive::prelude::*;

#[argio(output = AtCoder)]
fn main(x: i128, y: i128, a: i128, b: i128) -> i128 {
    let mut current_power = x;
    let mut i = 0;
    while current_power <= y {
        if current_power.checked_mul(a).unwrap_or(i128::MAX)
            > current_power.checked_add(b).unwrap_or(i128::MAX)
        {
            i += (y - current_power) / b;
            current_power = current_power + (y - current_power) / b * b;
            break;
        } else {
            current_power = current_power.checked_mul(a).unwrap_or(i128::MAX);
            i += 1;
        };
    }

    if current_power >= y {
        i - 1
    } else {
        i
    }
}
