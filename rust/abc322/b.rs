// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        _n: usize,
        _m: usize,
        s: String,
        t: String,
    }

    let is_prefix = t.starts_with(&s);
    let is_suffix = t.ends_with(&s);

    match (is_prefix, is_suffix) {
        (true, true) => println!("0"),
        (true, false) => println!("1"),
        (false, true) => println!("2"),
        (false, false) => println!("3"),
    }
}
