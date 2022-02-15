mod filtering;
mod information;
pub use std::cmp;

pub fn get_next_guess(word_vector: &mut Vec<&str>, input: &mut String, previous_guess: &mut String) -> Vec<String>{
    let mut current_included_ch: Vec<char> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let current_char = previous_guess.chars().nth(i).unwrap();
        match c {
            'x' => {
                *word_vector = filtering::filter_definite_ch(word_vector.to_vec(), i, current_char);
                current_included_ch.push(current_char);
            },
            'i' => {
                *word_vector = filtering::filter_indefinite_ch(word_vector.to_vec(), i, current_char);
                current_included_ch.push(current_char);
            },
            'n' => {
                if !current_included_ch.contains(&current_char) {
                    *word_vector = filtering::filter_indefinite_ch(word_vector.to_vec(), i, current_char);
                }
            },
            _ => {
                return Vec::new();
            }
        }
    }
    
    println!("{}", word_vector.len());
    
    let index = word_vector.iter().position(|x| *x == previous_guess);
    if !index.is_none() {
        word_vector.remove(index.unwrap());
    }
    
    information::sort_by_data(word_vector);  
    assert!(word_vector.len() != 0);

    let mut top_five: Vec<String> = Vec::new();
    for i in 0..cmp::min(word_vector.len(), 5) {
        top_five.push(word_vector[i].to_string());
    }

    return top_five;
}
