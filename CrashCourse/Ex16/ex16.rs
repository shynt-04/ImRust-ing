// Topic: Option

struct Student {
    name: String,
    locker_assignment: Option<i32>
}

fn main() {
    let my_student = vec![
        Student {
            name: "A".to_owned(),
            locker_assignment: Some(1101),
        },
        Student {
            name: "B".to_owned(),
            locker_assignment: None,
        },
    ];

    for student in my_student {
        match student.locker_assignment {
            Some(number) => println!("Student: {:?}, locker assignment: {:?}", student.name, number),
            None => println!("Student: {:?}, no locker assignment", student.name),
        }
    }

}