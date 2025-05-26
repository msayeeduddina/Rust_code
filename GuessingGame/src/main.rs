// use rand::seq::SliceRandom;
// use rand::thread_rng;
// use std::io;

// fn main() {
//     // Game initialization: Define the set of possible answers.
//     let fruits: Vec<&str> = vec![
//         "apple",
//         "banana",
//         "cherry",
//         "date",
//         "elderberry",
//         "fig",
//         "grape",
//     ];

//     // Core game logic: Randomly select the secret fruit.
//     let random_fruit = fruits
//         .choose(&mut thread_rng())
//         .expect("Failed to choose a fruit. The fruit list might be empty.");

//     // User interaction: Prompt for the guess.
//     println!("I've chosen a fruit. Can you guess it? {}", random_fruit);

//     // Game loop: Continually ask for user input until a correct guess or critical error.
//     loop {
//         let mut input = String::new();
//         println!("Enter your guess: ");

//         // Input handling: Read user input and manage potential I/O errors.
//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 // Data cleaning: Remove leading/trailing whitespace and newline characters from input.
//                 let input = input.trim();

//                 // Validation: Check if the input is empty to prevent invalid guesses.
//                 if input.is_empty() {
//                     eprintln!("Error: Your guess cannot be empty. Please enter a fruit name.");
//                     continue;
//                 }

//                 // Comparison logic: Determine if the user's guess matches the secret fruit.
//                 if check_ans(input, random_fruit) {
//                     // Game outcome: Notify the user of a correct guess and terminate the loop.
//                     println!("Congratulations! You guessed correctly: {}", random_fruit);
//                     break;
//                 } else {
//                     // Feedback loop: Inform the user of an incorrect guess and prompt for retry.
//                     println!("Sorry, your guess is incorrect. Try again!");
//                 }
//             }
//             Err(error) => {
//                 // Error handling: Report critical input reading errors and exit the game loop.
//                 eprintln!("Error reading input: {}", error);
//                 break;
//             }
//         }
//     }
// }

// /// Utility function: Performs a case-insensitive comparison between the user's input and the target fruit.
// fn check_ans(input: &str, fruit: &str) -> bool {
//     // String comparison: Utilize a method for robust, case-agnostic matching.
//     input.eq_ignore_ascii_case(fruit)
// }

//////////////////////////////////////////

use rand::prelude::*;
use std::io; // This brings in Rng, thread_rng, and SliceRandom

fn main() {
    // Game Setup: Define the list of fruits available for guessing.
    // Using a mutable array, though for a fixed list, `Vec<&str>` is often more flexible.
    let guess_fruitlist = [
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
    ];
    // Random Fruit Selection:
    // 1. Initialize the random number generator.
    let mut rng = thread_rng();
    // 2. Choose a random fruit from the list.
    //    `choose()` is preferred as it directly selects an item.
    let random_fruit = guess_fruitlist
        .choose(&mut rng)
        .expect("Failed to choose a fruit. The fruit list might be empty.");
    // User Prompt: Inform the user that a fruit has been chosen.
    println!("I've chosen a fruit. Can you guess it? {}", random_fruit);
    // Game Loop: Continue prompting the user until they guess correctly or an error occurs.
    loop {
        // Input Buffer: Create a new String to store each guess, preventing accumulation of previous inputs.
        let mut input = String::new();
        // Prompt for Guess: Display the prompt to the user.
        print!("Your guess: ");
        // Read Input: Attempt to read a line from standard input.
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Input Cleaning: Trim whitespace and convert the input to lowercase for case-insensitive comparison.
                let selected_fruit = input.trim().to_lowercase();
                // List Check (Optional but good for feedback): See if the guessed fruit is even in our known list.
                // Note: `as_str()` is needed to convert `String` to `&str` for comparison with array elements.
                if !guess_fruitlist.contains(&selected_fruit.as_str()) {
                    println!("'{}' is not in our list of fruits.", selected_fruit);
                    // Decided to let them try again rather than counting it as a loss immediately.
                }
                // Guess Evaluation: Check if the user's guess matches the randomly chosen fruit.
                // The `guess_checker` function handles case-insensitivity.
                if guess_checker(&selected_fruit, random_fruit) {
                    println!("Congratulations! You guessed correctly: {}", random_fruit);
                    break; // Exit the loop if the guess is correct.
                } else {
                    println!(
                        "Sorry, your guess '{}' is incorrect. Try again!",
                        selected_fruit
                    );
                }
            }
            Err(error) => {
                // Error Handling: Report any critical errors encountered during input reading.
                eprintln!("Error reading input: {}", error);
                break; // Exit the loop on an unrecoverable input error.
            }
        }
    }
}

// Helper Function: Compares the guessed fruit with the correct fruit, ignoring case.
fn guess_checker(guess_f: &str, corr_f: &str) -> bool {
    // String Comparison: Perform a case-insensitive comparison.
    guess_f.eq_ignore_ascii_case(corr_f)
}
