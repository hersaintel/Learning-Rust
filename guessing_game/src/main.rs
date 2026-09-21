use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn compare_guess(guess: u32, secret_number: u32) -> Ordering {
    guess.cmp(&secret_number)
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match compare_guess(guess, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_guesses_without_revealing_the_secret() {
        assert_eq!(compare_guess(25, 50), Ordering::Less);
        assert_eq!(compare_guess(75, 50), Ordering::Greater);
        assert_eq!(compare_guess(50, 50), Ordering::Equal);
    }
}
