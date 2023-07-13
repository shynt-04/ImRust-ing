// Topic: Arithmetic

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result() {
    println!("{:?}", sum(100, -1231));
}

fn main() {
    display_result();
}