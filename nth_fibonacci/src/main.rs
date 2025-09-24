use std::io;

fn main() {
    println!("Welcome to the nth_fibonacci finder!");
    println!("Please enter a numerical value for n to find the nth fibonacci");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input: u32 = user_input.trim().parse().expect("Please type a number");

    calculate_fibonacci(user_input);
    
}

fn calculate_fibonacci(n: u32) {
    let mut counter = 0;
    let mut prev = 0;
    let mut curr = 1;

    loop {
        if counter == 0 {
            println!("Fib {counter} = {prev}");
            counter += 1;
            continue;
        }

        if counter == 1 {
            println!("Fib {counter} = {curr}");
            counter += 1;
            continue;
        }

        let next = prev + curr;
        println!("Fib {counter} = {next}");

        prev = curr;
        curr = next;
        counter += 1;

        if counter > n {
            break;
        }
    }
}
