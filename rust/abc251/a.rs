// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", s.repeat(6 / s.len()));
}
