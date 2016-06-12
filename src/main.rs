use std::io;

fn main() {
  println!("Guess the number!");

  println!("Please input your guess.");

  let mut guess = String::new();

  io::stdin().read_line(&mut guess) //Returns a Result object that must have an error handler
    .ok() //for compatibility with Rust v1.0.0 (where expect isn't a method on Result)
    .expect("Failed to read line"); //The expect() handler crashes/panics(exits) the program

  println!("You guess: {}", guess);
}
