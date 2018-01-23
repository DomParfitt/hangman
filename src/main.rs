use std::io::{self, BufRead};

fn main() {

    let answer = "answer";
    let mut guessed_letters: Vec<&str> = Vec::new();

    print_current_guess(&guessed_letters, answer);
    get_user_guess(&guessed_letters);


}

fn print_current_guess(&guessed_letters: Vec<&str>, answer: &str) {
    for idx in 0..answer.len() {
        let letter = &answer[idx..idx+1];
        if guessed_letters.contains(&letter) {
            print!("{}", letter);
        } else {
            print!("_");
        }
        print!(" ");
    }
}

fn get_user_guess(&mut guessed_letters: Vec<&str>) {

    let mut input = String::new();
    let stdin = io::stdin();
    match stdin.lock().read_line(&mut input) {
        Ok(_) => {
            let letter = &input[0..1];
            if !guessed_letters.contains(&letter) {
                guessed_letters.push(letter);
            }
        },
        Err(_) => {
            println!("Please enter a letter.")
        },
    }
}
