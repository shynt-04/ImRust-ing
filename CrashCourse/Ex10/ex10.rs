// Topic: Expression
// print messages

fn print_message(greater: bool) {
    match greater {
        true => println!("its big"),
        _ => println!("its small"),
    }
}

fn main() {
    let my_variable = 1000;
    let greater = my_variable > 100;
    
    print_message(greater);
}