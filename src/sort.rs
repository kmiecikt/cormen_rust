pub fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in (1..list.len()).rev() {
        for j in 0..i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_simple() {
        run_simple_test(bubble_sort);
    }

    #[test]
    fn bubble_sort_reversed() {
        run_reversed_test(bubble_sort);
    }

    fn run_simple_test(tested_function: fn(&mut Vec<i32>) -> ()) {
        let mut input = vec![1, 4, 2, 8, 10, 3, 1];
        let expected = vec![1, 1, 2, 3, 4, 8, 10];

        tested_function(&mut input);

        assert_eq!(expected, input);
    }

    fn run_reversed_test(tested_function: fn(&mut Vec<i32>) -> ()) {
        let mut input = vec![5, 4, 3, 3, 2, 1, 1];
        let expected = vec![1, 1, 2, 3, 3, 4, 5];

        tested_function(&mut input);

        assert_eq!(expected, input);
    }
}