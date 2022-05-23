// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        w: i32,
        a: [i32; n],
    }

    let mut set = HashSet::new();
    for (i, ival) in a.iter().enumerate() {
        if ival <= &w {
            set.insert(*ival);
        }
        for (j, jval) in a.iter().enumerate() {
            if j <= i {
                continue;
            }

            let sum2 = ival + jval;
            if sum2 <= w {
                set.insert(sum2);
            }
            for (k, kval) in a.iter().enumerate() {
                if k <= j {
                    continue;
                }

                let sum3 = ival + jval + kval;
                if sum3 <= w {
                    set.insert(sum3);
                }
            }
        }
    }

    println!("{}", set.len());
}
