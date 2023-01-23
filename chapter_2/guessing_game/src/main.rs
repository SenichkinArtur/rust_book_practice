use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(0..100);

    println!("Guessing game");

    loop {
        let mut guess = String::new();

        println!("Please enter your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Entered value is less"),
            Ordering::Greater => println!("Entered values is greater"),
            Ordering::Equal => {
                println!("Entered value is equal");

                break;
            }
        }
    }
}
