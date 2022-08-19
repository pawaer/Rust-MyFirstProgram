use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number!:");

        let mut entered_number = String::new();

        io::stdin().read_line(&mut entered_number).expect("Failed To Read line");

        let entered_number: u32 = match entered_number.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        println!("You entered {entered_number}");

        match entered_number.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Jackpot!");
                break;
            }
        }
    }
}
