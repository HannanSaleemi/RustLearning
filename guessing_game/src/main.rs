// Use the Input/Output Library
use std::io;

// Main function that runs first
fn main() {

    // Simple Outputs
    println!("Guess the number");
    println!("Please input your guess:");

    // let - creates a variable
    // By default varibales are immutable - We have specified that this can be changed/mutable
    // Set type to string
    let mut guess = String::new();

    // Using the stdin function
        // & - Makes it a reference so multiple parts of your code access one peice of data
        // References are immutable by default - so set to mutable
        // Takes in parameter to assign the input to

        // Returns a io::Result - Is an enumeration (enum) - Contains values called "variants"
        // For Result - Variants are `Ok` or `Err`.
            // `Ok` - Operation Successful
            // `Err` - Failed and contains info on why failed
        // .except() function will cause program to crash and show message of failure
            // If not called - Rust will throw a warning telling you that a possible error hasn't been handeled
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line")
    
    // Print with placeholders
        // When you have multiple - Make sure they are in the correct order
    println!("You Guessed: {}", guess)
}