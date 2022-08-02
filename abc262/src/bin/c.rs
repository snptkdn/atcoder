use proconio::input;
use std::cmp::*;

fn main() {
    input!(N: usize, vec: [usize; N]);

    let mut count = 0;

    //println!("{:?}", vec);

    for i in vec.iter().enumerate() {
        for j in vec[i.0 + 1..].iter().enumerate() {
            let min = min(*i.1, *j.1);
            let max = max(*i.1, *j.1);

            //println!("min:{} max:{} i:{} j:{}", min, max, i.0 + 1, i.0 + j.0 + 2);
            if min == i.0 + 1 && max == i.0 + j.0 + 2 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
