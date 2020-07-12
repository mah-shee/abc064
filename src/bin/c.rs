#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],

    }
    let mut map = vec![0; 9];
    for i in 0..n {
        if a[i] >= 3200 {
            map[8] += 1;
            continue;
        }
        map[(a[i] / 400)] += 1;
    }
    let mut max = 0;
    let mut min = 0;
    for k in 0..8 {
        if map[k] > 0 {
            max += 1;
            min += 1;
        }
    }
    if map[8] > 0 {
        max += map[8];
        if min == 0 {
            min = 1;
        }
    }
    println!("{} {}", min, max);
}
