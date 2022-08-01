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

pub fn min_max<'a, T: PartialOrd>(values: &'a Vec<T>) -> Option<(&'a T, &'a T)> {
    let mut min: &'a T;
    let mut max: &'a T;
    let mut start_index;

    if values.len() == 0 {
        return None;
    }
    else if values.len() % 2 == 0 {
        start_index = 2;
        if values[0] <= values[1] {
            min = &values[0];
            max = &values[1];
        }
        else {
            min = &values[1];
            max = &values[0];
        }
    }
    else {
        start_index = 1;
        min = &values[0];
        max = &values[0];
    }
    
    while start_index < values.len() {
        let first = &values[start_index];
        let second = &values[start_index + 1];
        start_index += 2;
        
        if first <= second {
            if first < min {
                min = first;
            }
            if second > max {
                max = second;
            }
        }
        else {
            if second < min {
                min = second;
            }
            if first > max {
                max = first;
            }
        }
    }
    
    Some((min, max))
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
    
    #[test]
    fn min_max_odd() {
        let input = vec![1, 3, 5];
        let actual = min_max(&input);
        
        assert_eq!(Some((&1, &5)), actual);
    }

    #[test]
    fn min_max_even() {
        let input = vec![1, 5, 2, 0];
        let actual = min_max(&input);
        
        assert_eq!(Some((&0, &5)), actual);
    }
    
    #[test]
    fn min_max_empty() {
        let input: Vec<i32> = Vec::new();
        let actual = min_max(&input);

        assert_eq!(None, actual);
    }
}