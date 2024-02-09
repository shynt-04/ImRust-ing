use std::{io, process::exit};

fn check_flag(flag: &str) -> bool {
    if flag.len() < 18 {
        return false;
    }
    let check = flag.as_bytes()[1] as i32 & flag.as_bytes()[8] as i32 * flag.as_bytes()[0] as i32 + flag.as_bytes()[7] as i32 == 17 &&
    flag.as_bytes()[16] as i32 - flag.as_bytes()[6] as i32 ^ flag.as_bytes()[14] as i32 - flag.as_bytes()[2] as i32 * flag.as_bytes()[10] as i32 == -12376 &&
    flag.as_bytes()[4] as i32 ^ flag.as_bytes()[11] as i32 * flag.as_bytes()[7] as i32 - flag.as_bytes()[13] as i32 == 3526 &&
    flag.as_bytes()[0] as i32 * flag.as_bytes()[17] as i32 + flag.as_bytes()[17] as i32 == 9628 &&
    flag.as_bytes()[6] as i32 & flag.as_bytes()[14] as i32 ^ flag.as_bytes()[5] as i32 - flag.as_bytes()[1] as i32 == 19 &&
    flag.as_bytes()[7] as i32 & flag.as_bytes()[9] as i32 * flag.as_bytes()[11] as i32 ^ flag.as_bytes()[9] as i32 - flag.as_bytes()[0] as i32 == -51 &&
    flag.as_bytes()[6] as i32 ^ flag.as_bytes()[0] as i32 & flag.as_bytes()[17] as i32 == 12 &&
    flag.as_bytes()[17] as i32 + flag.as_bytes()[6] as i32 ^ flag.as_bytes()[9] as i32 == 194 &&
    flag.as_bytes()[6] as i32 ^ flag.as_bytes()[14] as i32 * flag.as_bytes()[4] as i32 - flag.as_bytes()[4] as i32 * flag.as_bytes()[7] as i32 == -201 &&
    flag.as_bytes()[16] as i32 - flag.as_bytes()[6] as i32 - flag.as_bytes()[7] as i32 * flag.as_bytes()[12] as i32 & flag.as_bytes()[17] as i32 == 2 &&
    flag.as_bytes()[7] as i32 - flag.as_bytes()[12] as i32 * flag.as_bytes()[15] as i32 - flag.as_bytes()[6] as i32 == -5564 &&
    flag.as_bytes()[9] as i32 + flag.as_bytes()[7] as i32 * flag.as_bytes()[3] as i32 + flag.as_bytes()[2] as i32 == 4301 &&
    flag.as_bytes()[14] as i32 + flag.as_bytes()[10] as i32 - flag.as_bytes()[13] as i32 ^ flag.as_bytes()[13] as i32 == 91 &&
    flag.as_bytes()[5] as i32 ^ flag.as_bytes()[7] as i32 ^ flag.as_bytes()[14] as i32 + flag.as_bytes()[8] as i32 == 169 &&
    flag.as_bytes()[9] as i32 & flag.as_bytes()[4] as i32 - flag.as_bytes()[11] as i32 + flag.as_bytes()[2] as i32 == 112 &&
    flag.as_bytes()[6] as i32 - flag.as_bytes()[3] as i32 + flag.as_bytes()[6] as i32 == 110 &&
    flag.as_bytes()[8] as i32 + flag.as_bytes()[2] as i32 * flag.as_bytes()[9] as i32 ^ flag.as_bytes()[6] as i32 + flag.as_bytes()[9] as i32 == 12519 &&
    flag.as_bytes()[15] as i32 * flag.as_bytes()[16] as i32 + flag.as_bytes()[16] as i32 - flag.as_bytes()[4] as i32 == 5314 &&
    flag.as_bytes()[3] as i32 & flag.as_bytes()[1] as i32 ^ flag.as_bytes()[0] as i32 & flag.as_bytes()[6] as i32 * flag.as_bytes()[16] as i32 == 66 &&
    flag.as_bytes()[8] as i32 ^ flag.as_bytes()[2] as i32 ^ flag.as_bytes()[11] as i32 == 80;           
    check
}

fn main() {
    // flag: BKSEC{s1mPL3_3xprEs510nS}
    println!("The easiest flag-checker xDD");
    loop {
        println!("Enter the flag: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        let flag = input.trim(); // remove whitespace from leading and trailing
        if check_flag(flag) {
            println!("Correct, now get tf out of here!!");
            exit(0)
        } else {
            println!("Wrong!!!");
        }
    }
}
