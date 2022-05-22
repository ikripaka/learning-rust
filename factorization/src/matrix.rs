use std::{fmt, io};
use std::error::Error;
use std::fmt::Formatter;
use std::mem::swap;
use rand::Rng;

#[derive(Clone)]
struct Matrix {
    matrix: Vec<Vec<u8>>,
    height: usize,
    length: usize,
}

#[derive(Clone)]
struct ResultTuple{
    pub is_complete: bool,
    pub value: u8,
}

impl Matrix {
    fn new(length: usize, height: usize) -> Self {
        let mut matrix: Vec<Vec<u8>> = vec![vec![0; length]; height];
        Matrix {
            length,
            height,
            matrix,
        }
    }

    fn change_matrix(&mut self, matrix: &Vec<Vec<u8>>) {
        self.matrix = matrix.clone()
    }

    fn transpose(&mut self) {
        let mut new_matrix: Vec<Vec<u8>> = vec![vec![0; self.height]; self.length];
        for i in 0..self.height {
            for j in 0..self.length {
                new_matrix[j][i] = self.matrix[i][j];
            }
        }
        self.length = self.height;
        self.height = new_matrix.len();
    }

    ///Solving matrix equation with Hauts method
    /// Ax=B ~ Ax=0
    fn solve_equations(&self) -> Result<Self, &str> {
        let mut b = Matrix::new(1, self.height);
        let mut a = copy_matrix(self);
        let mut x = vec![ResultTuple{is_complete: false, value: 0}; self.length];
        x[0].value = 0;
        x[0].is_complete = true;


        let mut x_matrix = Matrix::new(1,a.height - 1);

        for i in 0..a.height - 2 { //todo глянь чи правильно працює "-2"
            if a.matrix[i][i] != 1{
                match a.check_lines(&i) {
                    Ok(m) => a.swap_lines(i, m),
                    Err(_) => {
                        match a.check_columns(&i) {
                            Ok(m) => a.swap_columns(i, m),
                            Err(_) => return Err("impossible_to_solve")
                        };
                    }
                };
            }
            for j in i + 1..a.height-1 {
                let coef = a.matrix[j][i] & a.matrix[i][i];
                for k in 0..a.length {
                    a.matrix[j][k] = a.matrix[j][k] ^ a.matrix[i][k] * coef;
                }
            }
            println!("iteration:{}", i);
            println!("a matrix: \n{}", a);
            println!("b matrix: \n{}", b);
        }
        println!("final triangular a matrix: \n{}", a);

        //зворотний хід
        for i in (0..a.height-1).rev(){
            let mut sum = 0;
            for j in i..a.length{
                sum += a.matrix[i][j] & x_matrix.matrix[j][0];
            }
        }

        Ok(Matrix::new(1,4))
    }

    fn calc_weight(&self, line_index:usize) -> u128{
        let mut weight = 0;
        for j in 0..self.length{
            if self.matrix[line_index][j] == 1{
                weight += 1;
            }
        }
        weight
    }

    ///checking lines for presence of 'ones' for further swapping lines
    /// 0 1 0 => 1 0 0
    /// 1 0 0    0 1 0
    /// * * *    * * *
    fn check_lines(& self, i: &usize) -> Result<usize, bool>{
        for m in *i..self.height-1{
            if self.matrix[m][*i] == 1{
                return Ok(m);
            }
        }
        Err(false)
    }

    ///checking rows for presence of 'ones' for furhter swapping rows
    /// 0 1 0 => 1 0 1
    /// * * *    * * *
    fn check_columns(& self, i: &usize) -> Result<usize, bool> {
        for m in *i..self.length{
            if self.matrix[*i][m] == 1{
                return Ok(m);
            }
        }
        Err(false)
    }

    fn print_matrix(&self) {
        println!("{:?}", self.matrix)
    }

    fn swap_columns(&mut self, first_row_index: usize, second_row_index: usize) {
        let mut temp;
        for i in 0..self.height {
            temp = self.matrix[i][first_row_index];
            self.matrix[i][first_row_index] = self.matrix[i][second_row_index];
            self.matrix[i][second_row_index] = temp;
        }
    }

    fn swap_lines(&mut self, first_line_index: usize, second_line_index: usize) {
        let mut temp;
        for j in 0..self.length {
            temp = self.matrix[first_line_index][j];
            self.matrix[first_line_index][j] = self.matrix[second_line_index][j];
            self.matrix[second_line_index][j] = temp;
        }
    }
}

///makes matrix size k+1 * k
/// where the last line contains x_i indexes
/// that would be needed to solve matrix
/// 1 0
/// 0 1
/// 1 2
fn copy_matrix(input_matrix: &Matrix) -> Matrix {
    let length = input_matrix.length;
    let height = input_matrix.height + 1;
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; length]; height];

    for i in 0..height {
        if i == height - 1 {
            for k in 0..length {
                matrix[i][k] = k as u8;
            }
        } else {
            for j in 0..length {
                matrix[i][j] = input_matrix.matrix[i][j];
            }
        }
    }
    Matrix {
        length,
        height,
        matrix,
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s: String = String::new();
        for i in 0..self.matrix.len() {
            s.push_str(&(self.matrix.get(i).unwrap() as &Vec<u8>).into_iter()
                .map(|n| { n.to_string() + " " })
                .collect::<String>());
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
// impl ToString for Matrix {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {
    use crate::matrix::{Matrix};

    #[test]
    fn creating_matrix_test() {
        let mut matrix = Matrix::new(4, 10);
        matrix.print_matrix();
        matrix.transpose();
        assert_eq!((10, 4), (matrix.length, matrix.height))
    }

    #[test]
    fn clone_test() {
        let vector: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut cloned_vector = vector.clone();
        cloned_vector[0][1] = 7;
        assert_ne!(vector.get(0).unwrap(), cloned_vector.get(0).unwrap());
    }

    #[test]
    fn solve_equation1() {
        let vector: Vec<Vec<u8>> = vec![vec![0, 1, 0], vec![1, 0, 1], vec![1, 1, 1]];
        let mut a = Matrix::new(3, 3);
        a.change_matrix(&vector);
        println!("{:?}", a.matrix);
        a.solve_equations();
    }

    #[test]
    fn swap_lines_test() {
        let vector: Vec<Vec<u8>> = vec![vec![1, 0, 1], vec![1, 1, 1], vec![0, 1, 0]];
        let mut a = Matrix::new(3, 3);
        a.change_matrix(&vector);
        a.swap_lines(0, 2);
    }

    #[test]
    fn swap_rows_test() {
        let vector: Vec<Vec<u8>> = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let mut a = Matrix::new(3, 3);
        a.change_matrix(&vector);
        a.swap_columns(0, 2);
    }

    #[test]
    fn print_test() {
        let vector: Vec<Vec<u8>> = vec![vec![1, 0, 1], vec![1, 1, 1], vec![0, 1, 0]];
        let mut a = Matrix::new(3, 3);
        a.change_matrix(&vector);
        println!("{}", a)
    }
}
