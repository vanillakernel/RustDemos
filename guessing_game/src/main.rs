use std::io;

fn main() {

println!("Guess the number!");

println!("Please input your guess.");

//create a new mutable string object
let mut guess = String::new();

// update guess string with input from terminal
// if nothing, throw the error.
io::stdin().read_line(&mut guess)
   .expect("Failed to read line.");

// {} is a placeholder for the variable?
println!("You guessed: {}", guess);

}
