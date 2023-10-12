// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut first_abc_index: isize = -1;

    for i in 0..n - 2 {
        let sub_string = &s[i..i + 3];
        if sub_string == "ABC" {
            first_abc_index = (i + 1) as isize;
            break;
        }
    }

    println!("{}", first_abc_index);
}
