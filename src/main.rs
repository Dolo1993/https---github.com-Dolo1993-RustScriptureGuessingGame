use rand::seq::SliceRandom;
use std::io::{self, Write};
use std::time::{Duration, Instant};

// Function to display the current state of the word 
fn display_word(word: &str, guessed_letters: &Vec<char>) -> String {
    word.chars()
        .map(|c| if guessed_letters.contains(&c) || guessed_letters.contains(&c.to_ascii_lowercase()) { c } else { '_' })
        .collect()
}

// Function to get a valid guess from the player
fn get_guess() -> String {
    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Trim whitespace and convert to uppercase to make it case-insensitive
        let guess = guess.trim().to_ascii_uppercase();

        if guess.chars().all(char::is_alphabetic) {
            return guess;
        } else {
            println!("Invalid input. Please enter a valid alphabetical character or a full word.");
            println!();
        }
    }
}

fn main() {
    // Array of secret words
    let secret_words = ["Nephi", "Jacob ", "Enos", "Jarom", "Mosiah", "Moroni", "Esther", "John", "Genesis", "Numbers", "Samuel"];

    // Choose a random word from the array
    let mut rng = rand::thread_rng();
    let chosen_word = secret_words.choose(&mut rng).unwrap().to_string();

    // Track guessed letters
    let mut guessed_letters: Vec<char> = Vec::new();

    // Maximum number of allowed incorrect guesses
    let max_attempts = 3;
    let mut incorrect_attempts = 0;

    // Set time limit to 60 seconds
    let time_limit = Duration::from_secs(60);
    let start_time = Instant::now();
    println!();
    println!();
    println!("Welcome to the Scripture Guessing Game!");
    println!("Number of guesses allowed: {}", max_attempts); 
    println!("{}", chosen_word);

    let mut correct_guess_made = false;

    while incorrect_attempts < max_attempts && !correct_guess_made {
        // Display the current state of the word
        let display = display_word(&chosen_word, &guessed_letters);
        println!("Word: {}", display);

        // Get a guess from the player
        let guess = get_guess();

        // Check if the guessed letter or word is in the word (case-insensitive)
        if chosen_word.to_ascii_uppercase().contains(&guess) {
            println!("Correct!");
            println!();
            correct_guess_made = true;
        } else {
            println!("Incorrect!");
            println!();
            incorrect_attempts += 1;
        }

        // Add the guessed letters to the list
        guessed_letters.extend(guess.chars());

        // Display the number of guesses left
        println!("Guesses left: {}", max_attempts - incorrect_attempts);

        // Check if the player has guessed all the letters
        if display_word(&chosen_word, &guessed_letters) == chosen_word {
            println!("Congratulations! You guessed the word: {}", chosen_word);
            println!();
            break; // Exit the loop and terminate the program
        }

        // Check if time limit has been reached
        if start_time.elapsed() >= time_limit {
            println!("Time's up! You ran out of time.");
            println!();
            break;
        }
    }

    // Check if the player has run out of attempts or made a correct guess
    if incorrect_attempts == max_attempts && !correct_guess_made && start_time.elapsed() < time_limit {
        println!("Sorry, you ran out of attempts. The correct word was: {}", chosen_word);
        println!();
    }
}
