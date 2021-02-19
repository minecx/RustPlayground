use std::io; // io library from std lib
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret);

    loop {
        println!("Please enter your guess!");

        let mut guess = String::new(); // vars default immutable. mut = make it mutable

        // std::io::stdin() // if the first line is not there
        io::stdin()
            .read_line(&mut guess)
            // expect handles potential failure. If ok return value, else raise error.
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // u32 = unsigned 32-bit number. i32 = signed number. so on for u64, etc.
        // trim() removes white space in the beginning and in the end
        // pattern matching in Haskell. Evaluates like if-elif-else.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input - please enter a number!");
                continue;
            },
        };

        // println!("One string: |{}|, trimmed: |{}|.", "  abc  ", "  abc  ".trim());

        match guess.cmp(&secret) {
            Ordering::Less => println!("Guess higher!"),
            Ordering::Greater => println!("Guess lower!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
