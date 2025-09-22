use std::io;

fn main() {
    // The nth fibonacci number is a number that follows this function: 
    // F(n) = F(n-1) + F(n-2), with F(0) = 0, F(1) = 1
    // Step 1: Take user input, parse it into a numerical value, handle the arms of error
    println!("Welcome to the nth_fibonacci finder!");
    println!("Please enter a numerical value for n to find the nth fibonacci");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input: u32 = user_input.trim().parse().expect("Please type a number");

    // Step 2: Apply the formula to the input, return the answer
    calculate_fibonacci(user_input);
    
}

/* Output log

Welcome to the nth_fibonacci finder!
Please enter a numerical value for n to find the nth fibonacci
4
The 4th fibonacci is: -13

thread 'main' panicked at src/main.rs:28:18:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/

fn calculate_fibonacci(n: u32) -> () {
    let mut counter = 0;
    let mut result = 0;
    loop {
        if counter == 0 {
            result = 0;
        }
        if counter == 1 {
            result = 1;
        }
        result = result - 1 + result - 2;
        counter += 1;
         if counter == n {
            println!("The {n}th fibonacci is: {result}")
        }
    }
}
