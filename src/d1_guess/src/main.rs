// this is a game where you need to guess a random number

// importing stuff
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // variables
    let mut tries = 1;
    let secret = rand::thread_rng().gen_range(0..=100);

    println!("guess the number!");
    loop {
        println!("try {tries} - your guess is: ");
        let mut guess = String::new();

        // input handling
        io::stdin()
            .read_line(&mut guess)
            // check in case of error
            .expect("failed to read line :(");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // checking guess
        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("too small!");
            },
            Ordering::Greater => {
                println!("too big!");
            },
            Ordering::Equal => {
                println!("you win!1!1!! you took {tries} tries to find the number.");
                break;
            }
        }
        tries += 1;
    }
}
