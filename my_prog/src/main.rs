use std::io;
use std::cmp;

fn read_line_with_numbers() -> Vec<i64> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("");
    let vec = buf
                .split_whitespace()
                .map(|x| x.parse::<i64>().expect(""))
                .collect::<Vec<i64>>();
    vec
}
fn main() {
    let inputs = read_line_with_numbers();

    let d1 = inputs[0];
    let d2 = inputs[1];
    let d3 = inputs[2];

    let mut res = cmp::min(d1 + d2 + d3, (d1 + d2) * 2);
    res = cmp::min(res, (d1 + d3) * 2);
    res = cmp::min(res, (d2 + d3) * 2);
    println!("{res}");
}