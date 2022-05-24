// -*- coding:utf-8-unix -*-

use proconio::input;
use std::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }

    let mut min = i32::MAX;

    for row in &a {
        for val in row {
            min = cmp::min(min, *val);
        }
    }

    let mut ans = 0;

    for row in &a {
        for val in row {
            ans += val - min;
        }
    }

    println!("{}", ans);
}
