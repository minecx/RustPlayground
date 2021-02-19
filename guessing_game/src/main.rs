use std::io; // io library from std lib

fn main() {
    println!("Guess the number!");

    println!("Please enter your guess!");

    let mut guess = String::new(); // vars default immutable. mut = make it mutable

    // std::io::stdin() // if the first line is not there
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
