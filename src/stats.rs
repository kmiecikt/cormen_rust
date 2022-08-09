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

// Not pretty, but faster version of finding both minimum and maximum element in a vector.
// While calling the obvious implementation requires 2 * (n - 1) comparisons, this one 
// requires 3 * ceil(n / 2).
pub fn min_max<'a, T: PartialOrd, I: Iterator<Item = &'a T>>(mut values: I) -> Option<(&'a T, &'a T)> {
    let mut min: &'a T;
    let mut max: &'a T;

    let first = values.next();
    let second = values.next(); 
    
    match (first, second) {
        (None, _) => return None,
        (Some(first_value), None) => return Some((first_value, first_value)),
        (Some(first_value), Some(second_value)) => {
            (min, max) = if first_value <= second_value {
                (first_value, second_value)
            }
            else {
                (second_value, first_value)
            }
        }
    }

    loop {
        let first = values.next();
        let second = values.next();

        match (first, second) {
            (None, _) => return Some((min, max)),
            (Some(first_value), None) => {
                if first_value < min {
                    min = first_value;
                }
                if first_value > max {
                    max = first_value;
                }
                
                return Some((min, max));
            },
            (Some(first_value), Some(second_value)) => {
                if first_value <= second_value {
                    if first_value < min {
                        min = first_value;
                    }
                    if second_value > max {
                        max = second_value;
                    }
                }
                else {
                    if second_value < min {
                        min = second_value;
                    }
                    if first_value > max {
                        max = first_value;
                    }
                }
            }
        }
    }
}

// Finds nth element in the collection.
pub fn nth_element<'a, T: Copy + PartialOrd>(index: usize, values: &'a mut Vec<T>) -> Option<&'a T> {
    let mut left = 0;
    let mut right = values.len() - 1;

    if index > right {
        return None;
    }
    
    loop {
        let middle_index = partition(values, left, right);        
        if index == middle_index {
            return Some(&values[middle_index]);
        }
        else if middle_index > index {
            right = middle_index - 1;
        }
        else {
            left = middle_index + 1;
        }
    }
}

fn partition<T: Copy + PartialOrd>(values: &mut Vec<T>, left: usize, right: usize) -> usize {
    let middle_index = (left + right + 1) / 2;
    let middle = values[middle_index];
    values.swap(left, middle_index);
    
    let mut i = left + 1;
    let mut j = right;
    
    while i <= j {
        while values[i] <= middle && i < j {
            i += 1;
        }

        while values[j] > middle {
            j -= 1;
        }
        
        if i < j {
            values.swap(i, j);
        }

        i += 1;
    }
    
    values.swap(left, j);
    j
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use rand::seq::SliceRandom;
    
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
        let actual = min_max(input.iter());
        
        assert_eq!(Some((&1, &5)), actual);
    }

    #[test]
    fn min_max_even() {
        let input = vec![1, 5, 2, 0];
        let actual = min_max(input.iter());
        
        assert_eq!(Some((&0, &5)), actual);
    }
    
    #[test]
    fn min_max_empty() {
        let input: Vec<i32> = Vec::new();
        let actual = min_max(input.iter());

        assert_eq!(None, actual);
    }
    
    #[test]
    fn nth_element_sorted() {
        let input : Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7];

        for i in 0..input.len() {
            let mut clone = input.clone();
            let actual = nth_element(i, &mut clone);
            assert_eq!(Some(&(i + 1)), actual);
        }
    }

    #[test]
    fn nth_element_reverese_sorted() {
        let input : Vec<usize> = vec![6, 5, 4, 3, 2, 1];

        for i in 0..input.len() {
            let mut clone = input.clone();
            let actual = nth_element(i, &mut clone);
            assert_eq!(Some(&(i + 1)), actual);
        }
    }
    
    #[test]
    fn nth_element_randomized() {
        const SIZE: usize = 1000;
        let mut input: Vec<usize> = (0..SIZE).collect();
        let mut rng = StdRng::seed_from_u64(312);
        input.shuffle(&mut rng);
        
        for i in 0..SIZE {
            let mut clone = input.clone();
            let actual = nth_element(i, &mut clone);
            assert_eq!(Some(&i), actual);
        }
    }
}