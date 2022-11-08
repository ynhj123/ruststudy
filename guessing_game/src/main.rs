use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    loop {
        println!("welcome to guessing number!");
        //start..=end
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("secret_num range in 1-100!");
        loop {
            println!("please input your guess.");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to reead line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("please input number!");
                    continue;
                }
            };
            println!("You guessed: {guess}");
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("too small!"),
                Ordering::Greater => println!("too bog!"),
                Ordering::Equal => {
                    println!("you win!");
                    break;
                }
            }
        }
    }
}
