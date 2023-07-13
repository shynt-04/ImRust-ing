// Topic: Ownership

struct Grocery {
    quantity: i32,
    id: i32,
}

fn display_quantity(grocery: &Grocery) {
    println!("The quantity is {:?}", grocery.quantity); 
}

fn display_id(grocery: &Grocery) {
    println!("The id is {:?}", grocery.id); 
}

fn main() {
    let my_grocery = Grocery {
        quantity: 12,
        id: 123,
    };
    display_quantity(&my_grocery);
    display_id(&my_grocery)
}