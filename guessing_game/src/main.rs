use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number:!");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");


    let random_number = rand::thread_rng().gen_range(0..=1);



    println!("You guessed: {guess}");

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Just right"),
    }
}
