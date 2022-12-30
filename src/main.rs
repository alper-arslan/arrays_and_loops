#![allow(unused_variables, dead_code)]

fn main() {
    let matrix: [[i32;3]; 3] = [
        [1, 2, 3], // 
        [4, 5, 6], 
        [7, 8, 9]
        ];
    pretty_print(&matrix);
    let transposed = transpose(&matrix);
    println!("------------");
    pretty_print(&transposed);
}

fn transpose<'a>(matrix: &'a [[i32; 3]; 3]) -> [[i32; 3]; 3]{
    let mut result = [[0; 3]; 3];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            result[j][i] = matrix[i][j]
        }
    }
    return result;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for item in matrix {
        println!("|{}, {}, {}|", item[0], item[1], item[2]);
    }
}
