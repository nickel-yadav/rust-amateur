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
    let mut count = 1;
    let mut result = 1;
    loop {
        result += user_input - count;
        println!("Result : {result}");
        count += 1;
        if count == user_input {
            println!("The {user_input}'th fibonacci number is: {result}");
            break
        }
    }
}

fn calculate_fibonacci(n) {
    // FIXME: Update logic and test results
}
