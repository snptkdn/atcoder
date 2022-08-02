use proconio::input;

fn main() {
    input!(r: i64);

    println!("{}", if r % 4 < 3 { r + (2 - r % 4) } else { r + 3 })
}
