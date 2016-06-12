extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("Please input your guess.");

  let mut guess = String::new();

  io::stdin().read_line(&mut guess) //Returns a Result object that must have an error handler
    .ok() //for compatibility with Rust v1.0.0 (where expect isn't a method on Result)
    .expect("Failed to read line"); //The expect() handler crashes/panics(exits) the program

  let guess: u32 = guess.trim().parse()
    .ok() //for compatibility with Rust v1.0.0
    .expect("Please type a number!");

  println!("You guessed: {}", guess);

  match guess.cmp(&secret_number){
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
  }
}
