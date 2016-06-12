use std::io;

fn main() {
  println!("Guess the number!");

  println!("Please input your guess.");

  let mut guess = String::new();

  io::stdin().read_line(&mut guess) //Returns a Result object that must have an error handler
    .expect("Failed to read line"); //The expect() handler crashes/panics(exits) the program

  println!("You guess: {}", guess);
}
