/* Using Crates */
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess a Number");
    let winning_number = rand::thread_rng().gen_range(1..10);
    println!("Winning Number is {}", winning_number);

    loop {
        let mut guess = String::new();
        /* Using mutable Reference of Variable `guess` */
        io::stdin().read_line(&mut guess).expect("Failed to read Input");

        /* Trim all Whitespaces and parse it as Number */
        let guess_as_number: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        match guess_as_number.cmp(&winning_number) {
            Ordering::Less => println!("{}", ("Number is too small").red()),
            Ordering::Equal => {
                println!("{}",("Number is correct").green());
                break;
            }
            Ordering::Greater => println!("{}", ("Number is too big").red()),
        }
    }
}
