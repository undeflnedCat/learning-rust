// this is a game where you need to guess a random number

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(0..=100);
    println!("guess the number!");
    // println!("the number i'm thinking of is *whispering* {secret}");
    println!("your guess is: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line :(");
    let guess: u32 = guess.trim().parse().expect("please type a number not whatever this is!");
    println!("you guessed {guess}");
    match guess.cmp(&secret) {
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("too big!"),
        Ordering::Equal => println!("you win!1!1!!")
    }
}
