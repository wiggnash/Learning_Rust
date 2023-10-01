use std::io;
fn main() {
    println!("Hello, world!");
    println!("Please Enter Your Guess");

    //creating a variable to store the input from the user
    let mut guess = String::new(); //UTF-8 encoded growable string

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
