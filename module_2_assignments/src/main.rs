fn most_frequent_word(text: &str) -> (String, usize) {
    let words = text.split_whitespace().collect::<Vec<&str>>();  // Split the text into words
    let mut word_count: Vec<(String, usize)> = Vec::new();  // Vec to store the word and its frequency
    let mut max_word = String::new();
    let mut max_count = 0;

    // Count the frequency of each word
    for &word in words.iter() {
        let mut found = false;
        for &mut (ref mut w, ref mut count) in word_count.iter_mut() {
            if w == word {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            word_count.push((word.to_string(), 1));  // Add the word with initial count
        }
    }

    // Find the word with the highest frequency
    for &(ref word, count) in &word_count {
        if count > max_count {
            max_count = count;
            max_word = word.clone();
        }
    }

    (max_word, max_count) // return tuple
}


fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {

    *total = (low..=high).step_by(step.try_into().unwrap()).sum::<i32>(); 
}


fn main() {

    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);

    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
