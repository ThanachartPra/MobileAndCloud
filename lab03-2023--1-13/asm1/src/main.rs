fn word(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}

fn unique(words: Vec<String>) -> Vec<String> {
    let mut unique_words = Vec::new();
    for word in words {
        if !unique_words.contains(&word) {
            unique_words.push(word);
        }
    }
    unique_words
}

fn count(words: Vec<String>) -> usize {
    words.len()
}

fn main() {
    let input_string = "this cat this bat this rat";
    println!("string : {}",input_string);
    let words = word(input_string);
    println!("word : {:?}",words);
    let unique_words = unique(words);
    println!("unique : {:?}",unique_words);
    let count_unique = count(unique_words);
    println!("count : {}", count_unique);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_word() {
        let input_string = "this cat this bat this rat";
        let expected_output = vec!["this", "cat", "this", "bat", "this", "rat"];
        assert_eq!(word(input_string), expected_output);
    }
    #[test]
    fn test_unique() {
        let input_words = vec!["this", "cat", "this", "bat", "this", "rat"];
        let input_words: Vec<String> = input_words.iter().map(|s| s.to_string()).collect();
        // เพื่อแปลง input_words ให้เป็น Vec<String>
        let expected_output = vec!["this", "cat", "bat", "rat"];
        assert_eq!(unique(input_words), expected_output);
    }
    #[test]
    fn test_count() {
        let input_words = vec!["this", "cat", "bat", "rat"];
        let input_words: Vec<String> = input_words.iter().map(|s| s.to_string()).collect();
        let expected_output = 4;
        assert_eq!(count(input_words), expected_output);
    }
}