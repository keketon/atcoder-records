// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: isize,
        x: isize,
        a: [isize;n-1]
    }

    let max = *a.iter().max().unwrap();
    let min = *a.iter().min().unwrap();
    let cur_sum = a.iter().sum::<isize>() - min - max;

    let res: isize = match x - cur_sum {
        diff if diff <= min => 0,
        diff if diff <= max => diff,
        _ => -1,
    };

    println!("{}", res);
}
