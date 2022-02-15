pub fn sort_by_data(word_vector: &mut Vec<&str>) {
    let mut letter_list: [usize; 26] = [0; 26];
    get_letter_ranks(word_vector.to_vec(), &mut letter_list);
    
    word_vector.sort_by(|a, b| {
        let a_score = get_bit_value(&letter_list, a);
        let b_score = get_bit_value(&letter_list, b);
        return a_score.partial_cmp(&b_score).unwrap();
    });
}

pub fn get_letter_ranks(word_vector: Vec<&str>, letter_list: &mut [usize; 26]) {
    for word in word_vector {
        for letter in word.chars() {
            let letter_index = letter as usize - 97;
            letter_list[letter_index] += 1;
        }
    }
}

pub fn get_bit_value(letter_list: &[usize; 26], word: &str) -> f64 {
    let mut bits: f64 = 0.0;
    let mut current_contributing_ch: Vec<char> = Vec::new();
    for letter in word.chars() {
        if !current_contributing_ch.contains(&letter) {
            let letter_weight: f64 = letter_list[letter as usize - 97] as f64;
            bits += -letter_weight.log2();
            current_contributing_ch.push(letter);
        }
    }
    return bits;
}
