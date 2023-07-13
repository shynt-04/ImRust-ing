// Topic: Struct

enum Flavor {
    Almond,
    Apple,
    Apricot,
}

struct Drink {
    flavor: Flavor,
    ounce: f64,
}

fn display(drink: Drink) {
    match drink.flavor {
        Flavor::Almond => println!("almond "),
        Flavor::Apple => println!("apple "),
        Flavor::Apricot => println!("apricot "),
    };

    println!("ounce {:?}", drink.ounce);
}

fn main() {
    let my_drink = Drink {
        flavor: Flavor::Almond,
        ounce: 0.02,
    };
    display(my_drink);
}