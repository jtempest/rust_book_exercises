use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let target = gen_target(100);
    loop {
        let guess_str = read_guess();
        let guess: u32 = match guess_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        let result = guess.cmp(&target);
        let msg = match result {
            Ordering::Less => "Too small",
            Ordering::Greater => "Too large",
            Ordering::Equal => "You win!",
        };

        println!("{}", msg);

        let has_won = result == (Ordering::Equal);
        if has_won {
            break;
        }
    }
}

fn gen_target(max_guess: u32) -> u32 {
    rand::thread_rng().gen_range(1, max_guess + 1)
}

fn read_guess() -> String {
    print("> ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    String::from(guess.trim())
}

fn print(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}
