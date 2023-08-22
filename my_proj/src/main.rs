use std::io;

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut user_input);
    println!("The link is: {}", user_input);
}
