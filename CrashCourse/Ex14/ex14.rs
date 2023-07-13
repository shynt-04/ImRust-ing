// Topic: Strings

struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn print_color(color: &str) {
    println!("fav color: {:?}", color);
}

fn main() {
    let person_list = vec![
        Person {
            age: 17,
            name: String::from("An"),
            color: "red".to_owned(), 
        },
        Person {
            age: 18,
            name: String::from("annotshy"),
            color: "blue".to_owned(),
        },
        Person {
            age: 19,
            name: String::from("shynt_"),
            color: "gray".to_owned(),
        },
    ];

    for person in person_list {
        if person.age < 20 {
            println!("age: {:?}", person.age);
            print_name(&person.name);
            print_color(&person.color);
        }
    }
}