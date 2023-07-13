// Topic: Vectors

fn main() {
    let my_vector = vec![10, 20, 30, 40];
    let mut sum = 0;

    for number in &my_vector {
        if number == &30 {
            println!("thirty");
        } else {
            println!("{:?}", number);
        }
        sum += number;
    }

    println!("The sum of numbers is {:?}", sum);
    println!("The length of vector is {:?}", my_vector.len());
}