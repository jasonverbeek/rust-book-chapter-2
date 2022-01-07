use rand::Rng;
use std::io::{self, Write};

fn get_diff(a: i32, b: i32) -> i32 {
    let diff: i32 = a - b;
    if diff < 0 {
        return -diff;
    }
    return diff;
}

fn main() -> std::io::Result<()> {
    let secret_number: i32 = rand::thread_rng().gen_range(0..101);
    let mut last_diff: i32 = -1;

    println!("Welcome to Guessr from chapter 2 of the rust book");
    println!("I added a funny twist, instead of telling you wether its higher or lower");
    println!("you'll receive a warmer or colder after your first guess");
    println!();
    loop {
        print!("Guess a number between 0-100: ");
        std::io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input)?;

        let guess: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        if guess == secret_number {
            println!("you guessed the number correctly! it was {}", secret_number);
            break;
        }

        match last_diff {
            -1 => {
                println!("Nice guess, but you are wrong, try again!");
                last_diff = get_diff(secret_number, guess);
            }
            _ => {
                let diff: i32 = get_diff(secret_number, guess);
                if diff < last_diff {
                    println!("getting warmer!")
                } else {
                    println!("getting colder!")
                }
                last_diff = diff;
            }
        }
    }
    Ok(())
}
