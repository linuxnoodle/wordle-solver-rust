extern crate colored;

pub use std::fs;
pub use std::io;
pub use std::io::Write;
pub use colored::*;

mod solving;

fn main() {
    // Very clean, I know
    // TODO: Find some other (preferably less garbage) way to do this?
    println!("{}{}{}{}{}{}{}", "Respond with 5 letters (x for exact ".purple(), "[meaning green]".green(), " i for included ".purple(), "[meaning yellow]".yellow(), " and n for none ".purple(), "[meaning grey]".truecolor(69, 69, 69), ")".purple());
    println!("\t{}{}", "Example: ".purple(), "xinni".yellow());

    // Loads words.txt
    let contents = fs::read_to_string("words.txt").expect(format!("Error reading file: words.txt").as_str());
    let mut word_vector : Vec<&str> = contents.split("\n").collect();

    let mut i = 1;
    println!("Initial guess: soare");
    while i < 7 {
        print!("{}", "Enter selected word: ".yellow());
        let _ = io::stdout()
            .flush()
            .expect("Could not flush stdout");

        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
        if word.ends_with('\n') {
            word.pop();
        }

        if word.len() != 5 {
            println!("{}", "Please enter a 5 letter word.".red());
        } else {
            print!("{} ", "> ".yellow());
            let _ = io::stdout()
                .flush()
                .expect("Could not flush stdout");
        
            // Read formatted response
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read next line");
            if input.ends_with('\n') {
                input.pop();
            }

            let last_five: Vec<String> = solving::get_next_guess(&mut word_vector, &mut input, &mut word);
        
            if last_five.is_empty() {
                println!("{}", "Either your input was incorrectly formatted (5 letters [x, i, n]), or there are no valid words that match these restrictions.".red().bold());
            } else {
                println!("Guess {}: {:?}", i, last_five);
                i += 1;
            }
        }
    }
}
