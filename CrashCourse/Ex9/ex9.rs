// Topic: Tuples

fn coordinate() -> (i32, i32) {
    (5, 1)
}

fn main() {
    let my_coordiante = coordinate();

    if my_coordiante.1 > 5 {
        println!("> 5");
    } else if my_coordiante.1 == 5 {
        println!("= 5");
    } else {
        println!("< 5");
    }
}