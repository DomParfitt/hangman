use std::io::{self, BufRead};

fn main() {
    let mut guesses_remaining: u8 = 5;
    let answer = String::from("answer");
    let mut guessed_letters: Vec<String> = Vec::new();

    println!("I'm thinking of a word with {} letters", answer.len());
    while guesses_remaining > 0 {
        print_current_guess(&guessed_letters, &answer);
        println!("You have {} guesses remaining", guesses_remaining);
        println!("Please enter a letter: ");
        match get_user_guess() {
            Some(guess) => {

                let mut correct: bool = false;
                for idx in 0..answer.len() {
                    let letter = &answer[idx..idx+1].to_string();
                    if letter == &guess {
                        correct = true;
                        break;
                    }
                }
                if !correct {
                    println!("Nope, the answer doesn't contain that letter.");
                    guesses_remaining -= 1;
                } else {
                    println!("Good guess!");
                }

                if !guessed_letters.contains(&guess) {
                    guessed_letters.push(guess);
                }

                if has_won(&guessed_letters, &answer) {
                    println!("Congratulations! You won!");
                    break;
                }

                if guesses_remaining == 0 {
                    println!("Sorry, you lost.");
                    println!("The correct answer was: {}", answer);
                }


            },
            None => {
                println!("Please enter a letter: ");
            },
        }

    }

}

fn print_current_guess(guessed_letters: &Vec<String>, answer: &String) {
    for idx in 0..answer.len() {
        let letter = &answer[idx..idx+1].to_string();
        if guessed_letters.contains(&letter) {
            print!("{}", letter);
        } else {
            print!("_");
        }
        print!(" ");
    }
    println!();
}

fn get_user_guess() -> Option<String> {
//    let input = String::new();
    let stdin = io::stdin();

    match stdin.lock().lines().next() {
        Some(res) => {
            match res {
                Ok(x) => {
                    if x.len() >= 1 {
                        return Some(x[0..1].to_string());
                    }

                },
                Err(_) => {
                    return None;
                },
            }
        },
        None => {
            return None;
        },
    }
    return None;
}

fn has_won(guessed_letters: &Vec<String>, answer: &String) -> bool {
    for idx in 0..answer.len() {
        let letter = &answer[idx..idx+1].to_string();
        if !guessed_letters.contains(&letter) {
            return false;
        }
    }
    return true;
}