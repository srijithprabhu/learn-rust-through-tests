use std::mem;
use std::fmt::{Display, Formatter, Result};

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

// Tuples can be used as function arguments and as return values
fn transpose(matrix: Matrix) -> Matrix {
    // `let` can be used to bind the members of a tuple to variables
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({0} {1})\n({2} {3})", self.0, self.1, self.2, self.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MatrixTestCase {
        matrix: Matrix,
        function: fn(Matrix) -> Matrix,
        expected: String
    }

    #[test]
    fn test_matrix_print() {
        for test_case in [
            MatrixTestCase {
                matrix: Matrix(1.1, 1.2, 2.1, 2.2),
                function: transpose,
                expected: String::from("(1.1 2.1)\n(1.2 2.2)")
            }
        ] {
            let matrix = (test_case.function)(test_case.matrix);
            assert_eq!(test_case.expected, format!("{}", matrix))
        }
    }

    #[test]
    fn test_arrays_and_slices() {
        // Fixed-size array (type signature is superfluous)
        let xs: [i32; 5] = [1, 2, 3, 4, 5];

        // All elements can be initialized to the same value
        let ys: [i32; 500] = [0; 500];

        // Indexing starts at 0
        println!("first element of the array: {}", xs[0]);
        println!("second element of the array: {}", xs[1]);

        // `len` returns the count of elements in the array
        println!("number of elements in array: {}", xs.len());

        // Arrays are stack allocated
        println!("array occupies {} bytes", mem::size_of_val(&xs));

        // Arrays can be automatically borrowed as slices
        println!("borrow the whole array as a slice");
        analyze_slice(&xs);

        // Slices can point to a section of an array
        // They are of the form [starting_index..ending_index]
        // starting_index is the first position in the slice
        // ending_index is one more than the last position in the slice
        println!("borrow a section of the array as a slice");
        analyze_slice(&ys[1 .. 4]);
    }
}
