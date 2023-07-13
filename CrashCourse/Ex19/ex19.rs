use std::collections::HashMap;

fn main() {
    let mut my_store = HashMap::new();
    my_store.insert("Chairs", 5);
    my_store.insert("Beds", 3);
    my_store.insert("Tables", 2);
    my_store.insert("Couches", 0);
    for (key, value) in my_store.iter() {
        match value {
            0 => println!("{} : out of stock", key),
            _ => println!("{} : {}", key, value),
        }
    }
}
