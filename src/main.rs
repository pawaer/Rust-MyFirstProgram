use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut entered_number = String::new();

        println!("Guess a number!:");

        io::stdin().read_line(&mut entered_number).expect("Failed To Read line");

        println!("You entered {entered_number}");
        println!("expected number {secret_number}");

        let entered_number: u32 = entered_number.trim().parse().expect("Please enter a number");

        println!("You entered {entered_number}");


        match entered_number.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => println!("Jackpot!"),
        }
    }
}
