// -*- coding:utf-8-unix -*-

use proconio::input;
use std::*;

fn main() {
    input! {
        n: usize,
    }

    let ans = (1..=n)
        .filter(|i| no_seven(&format!("{}", i)) && no_seven(&format!("{:o}", i)))
        .count();

    println!("{}", ans);
}

fn no_seven(s: &str) -> bool {
    s.chars().all(|c| c != '7')
}
