// -*- coding:utf-8-unix -*-

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut unsatisfiable_str = "satisfiable";
    let set: HashSet<_> = s.iter().collect();

    for ele in s.iter() {
        if ele.starts_with("!") {
            continue;
        }

        let inverted = format!("!{}", ele);

        if set.contains(&inverted) {
            unsatisfiable_str = ele;
        }
    }

    println!("{}", unsatisfiable_str);
}
