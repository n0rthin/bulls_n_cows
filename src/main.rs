use std::io;
use rand::Rng;

fn main() {
    let secret_number: String = rand::thread_rng().gen_range(1000..=9999).to_string();
    let mut bulls;
    let mut cows;

    println!("Welcome to Bulls and Cows game!
Objective: Guess a secret 4-digit number between 1000 and 9999 (inclusive).
Gameplay:
    1. Make a guess
    2. Game will respond witha amount of \"bulls\" and \"cows\" in your guess:
        - bull: correct digit in the right place.
        - cow: correct digit in the wrong place.
    3. If you feel stuck and want to see the secret number enter \"give up\".
Winning: Correctly guess the secret number.
Examples:
    Secret number: 1231
        Guess: 1234 -> 3 Bulls (1, 2, and 3 are correct and in the right position), 0 Cows
        Guess: 2111 -> 1 Bull (the last 1 is in the right position), 3 Cows (the 211 are correct but in the wrong positions)
    Secret number: 4557
        Guess: 4575 -> 2 Bulls (first 4 and second 5 are correct), 2 Cows (5 and 7 are correct but swapped)
Aim to adjust your guesses based on feedback to find the secret number with minimal attempts.");

    loop {
        bulls = 0;
        cows = 0;
        
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess = guess.trim();

        if guess == secret_number {
            println!("You won!");
            break;
        }

        if guess == "give up" {
            println!("{secret_number}");
            continue;
        }

        if guess.len() != 4 {
            println!("Guess must consists of 4 digits");
            continue;
        }

        for (guess_i, guess_ch) in guess.trim().chars().enumerate() {
            let mut is_bull: bool = false;
            let mut is_cow: bool = false;
            for (secret_i, secret_ch) in secret_number.chars().enumerate() {
                if secret_ch == guess_ch && secret_i == guess_i {
                    is_bull = true;
                    break;
                }
                
                if secret_ch == guess_ch && secret_i != guess_i {
                    is_cow = true;
                }
            }

            if is_bull {
                bulls += 1;
            } else if is_cow {
                cows += 1;
            }
        }

        println!("Bulls: {bulls}, Cows: {cows}");
    }
}
