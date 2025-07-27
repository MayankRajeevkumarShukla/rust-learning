use std::io;
fn main() {
    println!("Guess a number");
    println!("Please input your guess");

    let mut guess = string::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read a line")
    println!("you guessed:{guess}")    
}
