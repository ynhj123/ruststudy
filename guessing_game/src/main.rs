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
            let guess = input_guessing_num();
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

fn input_guessing_num() -> u32 {
    return loop {
        println!("please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        match guess.trim().parse() {
            Ok(num) => { break num; }
            Err(_) => {
                println!("please input number!");
                continue;
            }
        };
    };
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_tow() {
        assert_eq!(4, add_two(2))
    }
}
