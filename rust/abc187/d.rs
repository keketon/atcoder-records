// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        voter: [(i64, i64); n],
    }

    let mut voter = voter;
    let mut score: i64 = voter.iter().map(|t| -t.0).sum();

    voter.sort_by(|x, y| diff(x).cmp(&diff(y)).reverse());

    let mut i = 0;

    while score <= 0 {
        score += diff(&voter[i]);
        i += 1;
    }

    println!("{}", i);
}

fn diff(v: &(i64, i64)) -> i64 {
    2 * v.0 + v.1
}
