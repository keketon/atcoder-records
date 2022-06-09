// -*- coding:utf-8-unix -*-

use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i32; n],
        t: [i32; n], 
    }

    let mut q = BinaryHeap::new();

    for (i, time) in t.iter().enumerate() {
        q.push((-(*time), i));
    }

    let mut ans = vec![-1; n];
    let mut c = 0;

    while c < n {
        let (time, i) = q.pop().unwrap();
        if ans[i] < 0 {
            c += 1;
            ans[i] = -time;
        }

        q.push((time - s[i],(i+1) % n));
    }

    for ele in ans {   
        println!("{}", ele);
    }
}
