// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize,
    }

    let mut adj = 4;
    if r == 1 {
        adj -= 1;
    }
    if r == h {
        adj -= 1;
    }

    if c == 1 {
        adj -= 1;
    }

    if c == w {
        adj -= 1;
    }

    println!("{}", adj);
}
