use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number!:");

    let secretNumber = rand::thread_rng().gen_range(1..=100);

    let mut enteredNumber = String::new();

    io::stdin().read_line(&mut enteredNumber).expect("Failed To Read line");

    println!("You entered {enteredNumber},");
    println!("expected number {secretNumber},");
}
