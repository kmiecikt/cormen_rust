pub fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in (1..list.len()).rev() {
        for j in 0..i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

pub fn insert_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in 1..list.len() {
        for j in 0..i {
            if list[i] < list[j] {
                let temp = list[i];
                for k in (j..i).rev() {
                    list[k + 1] = list[k];
                }
                list[j] = temp;

                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    use rand::seq::SliceRandom;

    #[test]
    fn bubble_sort_simple() {
        run_simple_test(bubble_sort);
    }

    #[test]
    fn bubble_sort_reversed() {
        run_reversed_test(bubble_sort);
    }

    #[test]
    fn bubble_sort_randomized() {
        run_randomized_test(bubble_sort);
    }

    #[test]
    fn insert_sort_simple() {
        run_simple_test(insert_sort);
    }

    #[test]
    fn insert_sort_reversed() {
        run_reversed_test(insert_sort);
    }

    #[test]
    fn insert_sort_randomized() {
        run_randomized_test(insert_sort);
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

    fn run_randomized_test(tested_function: fn(&mut Vec<i32>) -> ()) {
        let mut input: Vec<i32> = (1..2048).map(|i| i / 2).collect();
        let expected = input.clone();

        let mut rng = StdRng::seed_from_u64(312);
        input.shuffle(&mut rng);

        tested_function(&mut input);

        assert_eq!(expected, input);
    }
}