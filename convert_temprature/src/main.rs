use std::io; 

fn main() {
    println!("Enter numeric value of temprature:");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input: u32 = match user_input.trim().parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Failed to parse, defaulting to 0");
            0
        }
    };
    
    println!("Enter 1 for C to F, 2 for F to C");
    let mut operation_type = String::new();
    io::stdin().read_line(&mut operation_type).expect("Failed to read line");
    let operation_type: u32 = match operation_type.trim().parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Invalid input, defaulting to 0");
            0
        }
    };

    if operation_type == 1 {
        let result = convert_to_farenheit(user_input);
        println!("{user_input} degree celsius is {result} degrees farenheit");
    } else {
        let result = convert_to_celsius(user_input);
        println!("{user_input} degree farenheit is {result} degrees celsius");
    }

}


fn convert_to_farenheit(temprature: u32) -> u32 {
    temprature * 9/5 + 32
}

fn convert_to_celsius(temprature: u32) -> u32 {
    temprature - 32 * 5/9
}
