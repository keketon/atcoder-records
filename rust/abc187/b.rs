// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut c = vec![];

    for _ in 0..n {
        input! {
            xi: i32,
            yi: i32,
        }

        c.push((xi, yi));
    }

    let mut ans = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = i32::abs(c[i].0 - c[j].0);
            let dy = i32::abs(c[i].1 - c[j].1);

            if dy <= dx {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
