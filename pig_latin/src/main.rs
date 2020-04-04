extern crate unicode_segmentation;

use std::io::{self, Write};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("Enter a phrase to pig-latinize it, or 'quit' to quit.");
    loop {
        let input = read_line();
        let input = input.trim();
        if input == "quit" {
            break;
        }

        let words = split_words(&input);
        for w in &words {
            let pig = igpay_atinlay(&w);
            print!("{} ", pig);
        }
        println!();
    }
}

fn read_line() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn split_words(input: &str) -> Vec<&str> {
    input.unicode_words().collect::<Vec<&str>>()
}

fn igpay_atinlay(word: &str) -> String {
    let first_vowel = word.graphemes(true).position(|c| is_vowel(c));
    match first_vowel {
        Some(idx) => {
            if idx == 0 {
                format!("{}hay", word)
            } else {
                let first = &word[..idx];
                let rest = &word[idx..];
                format!("{}{}ay", rest, first)
            }
        }
        None => String::from(word),
    }
}

// non-exhaustive
fn is_vowel(c: &str) -> bool {
    const VOWELS: &str = "aeiouy";
    VOWELS.contains(c)
}
