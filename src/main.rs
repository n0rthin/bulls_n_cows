use std::io;
use rand::Rng;

fn main() {
    let secret_number: String = rand::thread_rng().gen_range(1000..=9999).to_string();
    let mut bulls;
    let mut cows;

    loop {
        bulls = 0;
        cows = 0;
        
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == secret_number {
            println!("You won!");
            break;
        }

        if guess.trim() == "give up" {
            println!("{secret_number}");
            continue;
        }
        // if we determined one number as a bull we need to skip in all subsequent checks
        // because it can be marked both as bull and cow in cases one digit is repeated twice
        // For example: secret number is 6005 and guess is 5006
        let mut bulls_indexes: Vec<usize> = Vec::new();
        for (guess_i, guess_ch) in guess.trim().chars().enumerate() {
            for (secret_i, secret_ch) in secret_number.chars().enumerate() {
                if bulls_indexes.contains(&secret_i) {
                    continue;
                }

                if secret_ch == guess_ch && secret_i == guess_i {
                    bulls += 1;
                    bulls_indexes.push(secret_i);
                }
                
                if secret_ch == guess_ch && secret_i != guess_i {
                    cows += 1;
                }
            }   
        }

        println!("Bulls: {bulls}, Cows: {cows}");
    }
}
