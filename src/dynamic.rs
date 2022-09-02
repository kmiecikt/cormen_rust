use std::{fmt::Write, cmp::Ordering, mem::swap};

pub fn cut_rod(prices: &Vec<f64>) -> (f64, Vec<usize>) {
    let mut profit = vec![0.0; prices.len() + 1];
    let mut best_cuts = vec![0; prices.len()];
    
    for i in 0..prices.len() {
        profit[i + 1] = f64::MIN;
        for j in 0..(i + 1) {
            if profit[i + 1] < prices[j] + profit[i - j] {
                profit[i + 1] = prices[j] + profit[i - j];
                best_cuts[i] = j + 1;
            }
        }
    }
    
    let mut cuts = Vec::new();
    let mut remaining = prices.len();
    while remaining > 0 {
        cuts.push(best_cuts[remaining - 1]);
        remaining -= best_cuts[remaining- 1];
    }
    
    (profit[prices.len()], cuts)
}

#[inline]
pub fn idx(x: usize, y: usize, n: usize) -> usize {
    x + y * n
}

pub struct MatrixSize {
    pub rows: usize,
    pub cols: usize
}

pub struct MultiplicationOrder {
    pub multiplications: usize,
    pub matrices_count: usize,
    pub order: Vec<usize>
}

impl MultiplicationOrder {
    pub fn to_string(&self) -> String {
        enum StackOperation {
            WriteCharacter(&'static str),
            WriteMultiplication(usize, usize)
        }

        let mut stack = Vec::new();
        let mut result = String::new();
        
        stack.push(StackOperation::WriteMultiplication(0, self.matrices_count - 1));

        while let Some(operation) = stack.pop() {
            match operation {
                StackOperation::WriteCharacter(s) => {
                    write!(result, "{}", s).unwrap();
                },
                StackOperation::WriteMultiplication(left, right) => {
                    if left == right {
                        write!(result, "A{}", left).unwrap();
                    }
                    else {
                        // We are using stack, so the order has to be reversed
                        let split = self.order[idx(left, right, self.matrices_count)];
                        stack.push(StackOperation::WriteCharacter(")"));
                        stack.push(StackOperation::WriteMultiplication(split + 1, right));
                        stack.push(StackOperation::WriteCharacter("*"));
                        stack.push(StackOperation::WriteMultiplication(left, split));
                        stack.push(StackOperation::WriteCharacter("("));
                    }
                }
            }
        }
        
        result
    }
}

pub fn matrix_mul(matrices: &Vec<MatrixSize>) -> MultiplicationOrder {
    let n = matrices.len();
    let mut multiplications = vec![0; n * n];
    let mut order = vec![0; n * n];
    
    // Size of the matrices
    for i in 1..matrices.len() {
        for j in 0..matrices.len() - i {
            let index = idx(j, j + i, n);
            multiplications[index] = usize::MAX;
            for k in j..j + i {
                let this_split_cost = multiplications[idx(j, k, n)] + multiplications[idx(k + 1, j + i, n)] + matrices[j].rows * matrices[k].cols * matrices[j + i].cols;
                if this_split_cost < multiplications[index] {
                    multiplications[index] = this_split_cost;
                    order[index] = k;
                }
            }
        }
    }
    
    MultiplicationOrder { 
        multiplications: multiplications[idx(0, n - 1, n)],
        matrices_count: n, 
        order: order 
    }
}

pub fn knapsack<'a>(items: &'a Vec<KnapsackItem>, capacity: usize) -> OptimalKnapsack<'a> {
    let mut best_previous_values = vec![0.0; capacity + 1];
    let mut best_current_values = vec![0.0; capacity + 1];
    let mut chosen_items = vec![false; items.len() * capacity];

    for i in 0..items.len() {
        for j in 1..=capacity {
            best_current_values[j] = best_previous_values[j];

            if items[i].weight <= j {
                let with_item = best_previous_values[j - items[i].weight] + items[i].value;

                if with_item > best_current_values[j] {
                    best_current_values[j] = with_item;
                    chosen_items[idx(j - 1, i, capacity)] = true;
                }
            }
        }
        
        swap(&mut best_current_values, &mut best_previous_values);
    }    
    
    let (total_weight, best_value) = knapsack_find_best_value(&best_previous_values);
    let items = knapsack_find_items(items, &chosen_items, capacity, total_weight);
        
    OptimalKnapsack { total_weight: total_weight, total_value: best_value, items: items }
}

fn knapsack_find_best_value(values: &Vec<f64>) -> (usize, f64) {
    let (index, best_value) = values.iter().enumerate()
        .max_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap_or(Ordering::Equal))
        .unwrap();
    
    (index, *best_value)
}

fn knapsack_find_items<'a>(items: &'a Vec<KnapsackItem>, chosen_items: &Vec<bool>, capacity: usize, total_weight: usize) -> Vec<&'a KnapsackItem> {
    let mut current_weight = total_weight;
    let mut result = Vec::new();

    for i in (0..items.len()).rev() {
        if chosen_items[(idx(current_weight - 1, i, capacity))] {
            result.push(&items[i]);
            current_weight -= items[i].weight;
            
            if current_weight == 0 {
                break;
            }
        }
    }

    result.reverse();
    result
}

#[derive(Debug, PartialEq)]
pub struct KnapsackItem {
    pub value: f64,
    pub weight: usize
}

pub struct OptimalKnapsack<'a> {
    pub total_weight: usize,
    pub total_value: f64,
    pub items: Vec<&'a KnapsackItem>
}

pub fn longest_common_substring(x: &String, y: &String) -> String {
    let x_chars: Vec<char> = x.chars().collect();
    let y_chars: Vec<char> = y.chars().collect();
    
    let mut lcs_table = vec![LcsData { length: 0, longest_substring: LcsBestSubstring::Both }; x_chars.len() * y_chars.len()];

    fn get_lc_lengths(data: &Vec<LcsData>, i: usize, j: usize, n: usize) -> usize {
        if i == 0 || j == 0 {
            0
        }
        else {
            data[idx(i - 1, j - 1, n)].length
        }
    }

    for i in 1..=x_chars.len() {
        for j in 1..=y_chars.len() {
            let mut best = LcsData { length: 0, longest_substring: LcsBestSubstring::Both };
            if x_chars[i - 1] == y_chars[j - 1] {
                best.length = get_lc_lengths(&lcs_table, i - 1, j - 1, x_chars.len()) + 1;
                best.longest_substring = LcsBestSubstring::Both;
            }
            else {
                let best_i = get_lc_lengths(&lcs_table, i - 1, j, x_chars.len());
                let best_j = get_lc_lengths(&lcs_table, i, j - 1, x_chars.len());

                if best_i > best_j {
                    best.length = best_i;
                    best.longest_substring = LcsBestSubstring::X;
                }
                else {
                    best.length = best_j;
                    best.longest_substring = LcsBestSubstring::Y;
                }
            }

            lcs_table[idx(i - 1, j - 1, x_chars.len())] = best;
        }
    }

    // Walking back
    let mut result = Vec::new();
    let mut i = x.len();
    let mut j = y.len();

    while i > 0 && j > 0 {
        let current = &lcs_table[idx(i - 1, j - 1, x_chars.len())];
        match current.longest_substring {
            LcsBestSubstring::Both => {
                result.push(x_chars[i - 1]);
                i -= 1;
                j -= 1;
            },
            LcsBestSubstring::X => {
                i -= 1;
            },
            LcsBestSubstring::Y => {
                j -= 1;
            }
        }
    }

    result.reverse();

    result.into_iter().collect()
}

#[derive(Copy, Clone)]
enum LcsBestSubstring {
    Both,
    X,
    Y
}

#[derive(Copy, Clone)]
struct LcsData {
    length: usize,
    longest_substring: LcsBestSubstring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut_rod_single_cut_test() {
        let prices : Vec<f64> = vec![1.0, 5.0, 8.0, 10.0, 13.0, 17.0, 18.0, 22.0, 25.0, 30.0];
        let (profit, cuts) = cut_rod(&prices);

        assert_eq!(30.0, profit);
        assert_eq!(vec![10], cuts);
    }

    #[test]
    fn cut_rod_two_cuts_test() {
        let prices : Vec<f64> = vec![1.0, 5.0, 8.0, 10.0, 13.0, 17.0, 18.0];
        let (profit, cuts) = cut_rod(&prices);

        assert_eq!(18.0, profit);
        assert_eq!(vec![1, 6], cuts);
    }
    
    #[test]
    fn matrix_mul_test() {
        let data = vec![
            MatrixSize { rows: 30, cols: 35 },
            MatrixSize { rows: 35, cols: 15 },
            MatrixSize { rows: 15, cols: 5 },
            MatrixSize { rows: 5, cols: 10 },
            MatrixSize { rows: 10, cols: 20 },
            MatrixSize { rows: 20, cols: 25 }
        ];

        let result = matrix_mul(&data);
        
        assert_eq!(15125, result.multiplications);
        assert_eq!("((A0*(A1*A2))*((A3*A4)*A5))", result.to_string())
    }
    
    #[test]
    fn knapsack_1_test() {
        let items = vec![
            KnapsackItem { value: 1.0, weight: 2 },
            KnapsackItem { value: 4.0, weight: 3 },
            KnapsackItem { value: 5.0, weight: 6 },
            KnapsackItem { value: 6.0, weight: 7 }
        ];
        
        let expected_items = vec![
            &KnapsackItem { value: 4.0, weight: 3 },
            &KnapsackItem { value: 6.0, weight: 7 }
        ];

        let result = knapsack(&items, 10);
        assert_eq!(&expected_items, &result.items);
        assert_eq!(10.0, result.total_value);
        assert_eq!(10, result.total_weight);
    }
    
    #[test]
    fn knapsack_2_test() {
        let items = vec![
            KnapsackItem { value: 60.0, weight: 10 },
            KnapsackItem { value: 100.0, weight: 20 },
            KnapsackItem { value: 120.0, weight: 30 }
        ];

        let expected_items = vec![
            &KnapsackItem { value: 100.0, weight: 20 },
            &KnapsackItem { value: 120.0, weight: 30 }
        ];
        
        let result = knapsack(&items, 50);
        assert_eq!(&expected_items, &result.items);
        assert_eq!(220.0, result.total_value);
        assert_eq!(50, result.total_weight);
    }

    #[test]
    fn longest_common_substring_1_test() {
        let x = String::from("abcbdab");
        let y = String::from("bdcaba");

        let result = longest_common_substring(&x, &y);
        assert_eq!(String::from("bdab"), result);
    }
}