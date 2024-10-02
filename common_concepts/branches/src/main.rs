use std::io;

fn main() {
    let mut counter = 0;
    let result =  loop {
        println!("Please input your number."); 
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: u32 = match number.trim().parse() { 
            Ok(num) => num,
            Err(_) => {
                println!("It isn't a number: {number} please enter a number!");
                continue
            } 
        };
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
        let condition = false;
        let number = if condition { 5 } else { 6 };
    
        println!("The value of number is: {number}");
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The counter reached limits {result}");

}