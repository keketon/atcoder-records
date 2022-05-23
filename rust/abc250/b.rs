// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let na = n * a;
    let nb = n * b;

    let mut grid = vec![vec!['.'; nb]; na];

    for i in 0..na {
        for j in 0..nb {
            let parity = (i / a + j / b) % 2;
            if parity == 1 {
                grid[i][j] = '#';
            }
        }
    }

    for i in 0..na {
        let row: String = grid[i].iter().collect();
        println!("{}", row);
    }
}
