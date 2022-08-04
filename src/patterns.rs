/// Finds first occurrence of a pattern in a text using naive algorithm.
/// # Examples:
/// ```
/// use cormen_rust::patterns::naive_find;
///
/// assert_eq!(naive_find(&String::from("abc"), &String::from("abbabcdef")), vec![3]);
/// assert_eq!(naive_find(&String::from("abaab"), &String::from("abbabcabaabcd")), vec![6]);
/// assert_eq!(naive_find(&String::from("bab"), &String::from("aababab")), vec![2, 4]);
/// assert_eq!(naive_find(&String::from("abc"), &String::from("bdbdbdabd")), Vec::new());
/// ```
pub fn naive_find(pattern: &String, text: &String) -> Vec<usize> {
    let mut result = Vec::new();
    let pattern_vec: Vec<char> = pattern.chars().collect();
    if pattern_vec.len() == 0 {
        return result
    }

    let text_vec: Vec<char> = text.chars().collect();
    
    for i in 0..text_vec.len() - pattern_vec.len() + 1 {
        for j in 0..pattern_vec.len() {
            if pattern_vec[j] != text_vec[i + j] {
                break;
            }
            else if j == pattern_vec.len() - 1 {
                result.push(i);
            }
        }
    }
    
    result
}

/// Finds first occurrence of a pattern in a text using Knuth-Morris-Pratt algorithm.
/// # Examples:
/// ```
/// use cormen_rust::patterns::kmp_find;
///
/// assert_eq!(kmp_find(&String::from("abc"), &String::from("abbabcdef")), vec![3]);
/// assert_eq!(kmp_find(&String::from("abaab"), &String::from("abbabcabaabcd")), vec![6]);
/// assert_eq!(kmp_find(&String::from("bab"), &String::from("aababab")), vec![2, 4]);
/// assert_eq!(kmp_find(&String::from("abc"), &String::from("bdbdbdabd")), Vec::new());
/// ```
pub fn kmp_find(pattern: &String, text: &String) -> Vec<usize> {
    let pattern_vec: Vec<char> = pattern.chars().collect();
    let mut result = Vec::new();

    if pattern_vec.len() == 0 {
        return result;
    }

    let prefix_table  = create_kmp_prefix_table(&pattern_vec);
    let mut longest_prefix = 0;
    let mut i = 0;
    
    for text_char in text.chars() {
        i += 1;

        while longest_prefix > 0 && text_char != pattern_vec[longest_prefix] {
            longest_prefix = prefix_table[longest_prefix];
        }
        
        if text_char == pattern_vec[longest_prefix] {
            longest_prefix += 1;
        }
        
        if longest_prefix == pattern_vec.len() {
            result.push(i - pattern_vec.len());
            longest_prefix = prefix_table[longest_prefix - 1];
        }
    }
    
    result
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