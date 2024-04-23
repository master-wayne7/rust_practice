extern crate rand;
use std::io;

use rand::random;

fn get_guess() -> u8 {
    loop {
        println!("Input guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(_) => println!("Please enter a number"),
        }
    }
}

fn handle_guess(guess: u8, correct: u8) -> bool {
    if guess == correct {
        println!("You win!");
        return true;
    } else if guess > correct {
        println!("Too high!");
        return false;
    } else {
        println!("Too low!");

        return false;
    }
}

fn main() {
    let correct = random::<u8>();
    println!("welcome to the guessing game!");
    println!("Correct value: {correct}");
    loop {
        let guess = get_guess();
        if handle_guess(guess, correct) {
            break;
        }
    }
}
