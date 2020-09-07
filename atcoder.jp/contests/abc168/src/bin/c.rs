/**
 *    author : HikaruEgashira
 *    created: 09.07.2020 19.11
**/
use competitive::prelude::*;
use std::f64::consts::PI;

#[argio(output = AtCoder)]
fn main(a: f64, b: f64, h: f64, m: f64) -> f64 {
    let h_theta = h / 12_f64 + m / (60_f64 * 12_f64);
    let m_theta = m / 60_f64;
    let angle = (h_theta - m_theta) * 2_f64 * PI;

    (a * a + b * b - 2_f64 * a * b * angle.cos()).sqrt()
}
