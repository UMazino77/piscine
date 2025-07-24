use matrix_transposition::*;

fn main() {
    let matrix = Matrix((1, 3), (4, 5));

    println!("Original matrix {:?}", matrix.1.0);
    println!("Transpose matrix {:?}", transpose(matrix));
}