// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        h_1: usize,
        h_2: usize,
        h_3: usize,
        w_1: usize,
        w_2: usize,
        w_3: usize,
    }
    let mut ans = 0;
    for a_0 in 1..=h_1 {
        for a_1 in 1..=h_1 {
            for a_3 in 1..=h_2 {
                for a_4 in 1..=h_2 {
                    let a_2: isize = h_1 as isize - a_0 as isize - a_1 as isize;
                    let a_5: isize = h_2 as isize - a_3 as isize - a_4 as isize;
                    let a_6: isize = w_1 as isize - a_0 as isize - a_3 as isize;
                    let a_7: isize = w_2 as isize - a_1 as isize - a_4 as isize;
                    let a_8: isize = w_3 as isize - a_2 as isize - a_5 as isize;
                    if a_2 > 0 && a_5 > 0 && a_6 > 0 && a_7 > 0 && a_8 > 0 && a_6 + a_7 + a_8 == h_3 as isize {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
