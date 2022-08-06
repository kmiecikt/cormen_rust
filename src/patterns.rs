/// Finds first occurrence of a pattern in a text using Knuth-Morris-Prath algorithm.
/// # Examples:
/// ```
/// use cormen_rust::patterns::kmp_find;
///
/// assert_eq!(kmp_find(&String::from("abc"), &String::from("abbabcdef")), Some(3));
/// assert_eq!(kmp_find(&String::from("abaab"), &String::from("abbabcabaabcd")), Some(6));
/// assert_eq!(kmp_find(&String::from("abc"), &String::from("bdbdbdabd")), None);
/// ```
pub fn kmp_find(pattern: &String, text: &String) -> Option<usize> {
    let pattern_vec: Vec<char> = pattern.chars().collect();
    if pattern_vec.len() == 0 {
        return Some(0);
    }

    let mut longest_prefix = 0;
    let mut result = 0;
    let prefix_table  = create_kmp_prefix_table(&pattern_vec);
    
    for text_char in text.chars() {
        result += 1;

        while longest_prefix > 0 && text_char != pattern_vec[longest_prefix] {
            longest_prefix = prefix_table[longest_prefix];
        }
        
        if text_char == pattern_vec[longest_prefix] {
            longest_prefix += 1;
        }
        
        if longest_prefix == pattern_vec.len() {
            return Some(result - pattern_vec.len()); 
        }
    }
    
    None
}

// Prefix table: a mapping of a longest suffix of a sub-string 
// that is also its prefix.
fn create_kmp_prefix_table(pattern: &Vec<char>) -> Vec<usize> {
    let mut result = Vec::new();
    result.push(0);
    
    for i in 1..pattern.len() {
        let mut j = result[i - 1];
        while j > 0 && pattern[j] != pattern[i] {
            j = result[j];
        }
        
        if pattern[j] == pattern[i] {
            j += 1;
        }
        
        result.push(j);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefix_table_two_characters() {
        let pattern = vec!['a', 'b', 'a', 'a', 'b'];
        let prefix_table = create_kmp_prefix_table(&pattern);

        assert_eq!(vec![0, 0, 1, 1, 2], prefix_table);
    }
    
    #[test]
    fn prefix_table_single_character() {
        let pattern = vec!['a', 'a', 'a', 'a', 'a'];
        let prefix_table = create_kmp_prefix_table(&pattern);

        assert_eq!(vec![0, 1, 2, 3, 4], prefix_table);
    }
}