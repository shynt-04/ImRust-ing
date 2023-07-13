// Topic: Result

#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

impl Adult {
    fn new(name: &str, age: i32) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name : name.to_string(),
            })
        } else {
            Err("Under age")
        }
    }
}

fn main() {
    let child = Adult::new("A", 12);
    let adult = Adult::new("B", 22);
    match child {
        Ok(child) => println!("{} is {}", child.name, child.age),
        Err(e) => println!("{}", e),
    }
    match adult {
        Ok(adult) => println!("{} is {}", adult.name, adult.age),
        Err(e) => println!("{}", e),
    }
}