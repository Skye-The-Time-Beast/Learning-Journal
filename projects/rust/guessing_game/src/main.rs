// IO (input output library) being added from the standard libary (std)
use std::io;
// main function is the entry point into the program
//fn syntax is declaring a new function, the () indicating the parameters (in this case, none)
// { starts the body of a function
fn main() {
    // println! is a macro to print to screen
    println!("Guess the number!");

    println!("Please input your guess.");
    // "let" statement creates a variable
    // "mut" makes said variable 'mutable' meaning changeable and not fixed
    // "guess" is the name of the variable
    //you declare what type of variable, in this case a String
    //
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {guess}");
}
