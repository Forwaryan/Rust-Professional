/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement the logic to rotate the matrix 90 degrees in place
    let row = matrix.len();
    let col = matrix[0].len();
    if row <= col {
        for i in 0..col - row {
            matrix.push(vec![0; col]);
        }
    } else {
        for i in 0..row {
            matrix[i].extend(vec![0; row - col]);
        }
    }
    let n = matrix.len();

    //补全为方阵，然后上下翻转
    for i in 0..n {
        for j in 0..n / 2 {
            let temp = matrix[j][i];
            matrix[j][i] = matrix[n - j - 1][i];
            matrix[n - j - 1][i] = temp;
        }
    }
    for row in matrix.iter() {
        for &elem in row.iter() {
            print!("{:3} ", elem);
        }
        println!();
    }
    println!("----------------------");
    //是方阵并且直接按照主对角线翻转
    for i in 0..n {
        for j in 0..i {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }

    //区分删除左边还是右边的0，上边还是下边的0
    for row in matrix.iter() {
        for &elem in row.iter() {
            print!("{:3} ", elem);
        }
        println!();
    }

    if row <= col {
        /*
        1 2 3 4
        5 6 7 8
        9 10 11 12
        
        0 9 5 1
        0 10 6 2
        0 11 7 3
        0 12 8 4
         */
        for i in 0..n {
            for _j in 0..row - col {
                matrix[i].remove(0);
            }
        }
    } else {
        for _i in 0..row - col {
            matrix.pop();
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
