extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Gues the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please, input your guess...");

        let mut guess = String::new();
        // .expect catching returning Err from Result output and
        // prints parameter string.
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        if guess.trim() == "quit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guesed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
