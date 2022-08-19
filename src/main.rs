use std::io;

fn main() {
    println!("Guess a number!:");

    let mut enteredNumber = String::new();

    io::stdin().read_line(&mut enteredNumber).expect("Failed To Read line");

    println!("You entered {enteredNumber}");


    let x = 5;
    let y = -12;

    println!("You entered x {x} and y {}", y);
}
