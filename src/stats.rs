pub fn min<'a, T: PartialOrd, I: Iterator<Item = &'a T>>(values: I) -> Option<&'a T> {
    #[inline]
    fn fold<'a, T: PartialOrd>(first: &'a T, second: &'a T) -> &'a T {
        if first <= second { first } else { second }
    }
    
    values.reduce(fold)
}

pub fn max<'a, T: PartialOrd, I: Iterator<Item = &'a T>>(values: I) -> Option<&'a T> {
    #[inline]
    fn fold<'a, T: PartialOrd>(first: &'a T, second: &'a T) -> &'a T {
        if first >= second { first } else { second }
    }
    
    values.reduce(fold)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn min_for_not_empty() {
        let input = vec![1, 4, 5, -1, 2];
        let actual = min(input.iter());
        
        assert_eq!(Some(&-1), actual);
    }

    #[test]
    fn min_for_empty() {
        let input : Vec<i32> = Vec::new();
        let actual = min(input.iter());
        
        assert_eq!(None, actual);
    }

    #[test]
    fn max_for_not_empty() {
        let input = vec![1, 4, 5, -1, 2];
        let actual = max(input.iter());
        
        assert_eq!(Some(&5), actual);
    }

    #[test]
    fn max_for_empty() {
        let input: Vec<i32> = Vec::new();
        let actual = max(input.iter());
        
        assert_eq!(None, actual);
    }
}