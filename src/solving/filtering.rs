pub fn filter_definite_ch(word_list: Vec<&str>, index: usize, c: char) -> Vec<&str> {
    let mut new_list: Vec<&str> = Vec::new();
    for word in word_list {
        if word.chars().nth(index) == Some(c) {
            new_list.push(word);
        }
    }
    return new_list;
}

pub fn filter_indefinite_ch(word_list: Vec<&str>, index: usize, c: char) -> Vec<&str> {
    let mut new_list: Vec<&str> = Vec::new();
    for word in word_list {
        if word.chars().nth(index) != Some(c) {
            new_list.push(word);
        }
    }
    return new_list;
}
