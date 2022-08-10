use std::fmt::Write;

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
}