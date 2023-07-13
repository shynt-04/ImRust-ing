// Topic: While

fn main() {
    let mut i = 5;
    while i >= 1 {
        println!("{:?}", i);
        i -= 1;
    }
    println!("done");
}