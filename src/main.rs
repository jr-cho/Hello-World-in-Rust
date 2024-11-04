#[allow(unused_imports)]
use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let alp: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars().collect();

    let mut input = String::new();
    print!("Enter a String: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Could Not Read Line1");

    let input = input.trim();
    let mut display_text = String::new();

    let delay = time::Duration::from_millis(100);
    for target_char in input.chars() {
        for &c in &alp {
            print!("\r{}{}", display_text, c);
            io::stdout().flush().unwrap();

            if c == target_char {
                display_text.push(c);
                break;
            }

            thread::sleep(delay);
        }
    }
}
