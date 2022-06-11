// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut voter = Vec::new();
    let mut score = 0;

    for _ in 0..n {
        input! {
            a: i64,
            b: i64,
        }

        score -= a;
        voter.push((a, b));
    }

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
