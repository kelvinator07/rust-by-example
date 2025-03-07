use std::fmt::{self, Display, Formatter};
use std::mem;

fn main() {
    println!("========Primitives========");

    // Variables can be type annotated.
    let logical: bool = true;
    println!("logical > {}", logical);

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation
    println!("an_integer > {}", an_integer);

    // Or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`
    println!("default_integer > {}", default_integer);

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;
    println!("inferred_type > {}", inferred_type);

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    println!("mutable > {}", mutable);

    // Error! The type of a variable can't be changed to another type.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
    println!("mutable > {}", mutable);

    /* Compound types - Array and Tuple */

    // Array signature consists of Type T and length as [T; length].
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("my_array > {:?}", my_array);

    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);
    println!("my_tuple > {:?}", my_tuple);

    println!("\n\n========Literals and operators========");
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    println!("\n\n========Tuples========");

    // Tuples can be used as function arguments and as return values.
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // `let` can be used to bind the members of a tuple to variables.
        let (int_param, bool_param) = pair;
        (bool_param, int_param)
    }

    // A tuple with a bunch of different types.
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'z', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples (more than 12 elements) cannot be printed.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix: {:?}", matrix);

    // Adding fmt:Display trait to the Matrix
    impl Display for Matrix {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }

    let matrix_display = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix Fmt Display:\n{}", matrix_display);

    fn transpose(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }

    println!("Matrix Transpose:\n{}", transpose(matrix_display));

    println!("\n\n========Arrays and Slices========");

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5]; // 20 bytes total, 4 bytes per number in memory
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    // `len` returns the count of elements in the array.
    println!("Number of element in the array: {}", xs.len());
    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // This function borrows a slice.
    fn analyze_slize(slice: &[i32]) {
        println!("First element of the slice: {}", slice[0]);
        println!("Length of the slice: {}", slice.len());
    }

    // Arrays can be automatically borrowed as slices.
    println!("\nBorrow the whole array as a slice.");
    analyze_slize(&xs);

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("\nBorrow a section of the array as a slice.");
    analyze_slize(&ys[1..4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(element) => println!("{}: {}", i, element),
            None => println!("Slow down! Index {} is too far!", i),
        }
    }

    // Out of bound indexing on array with constant value causes compile time error.
    // println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println!("{}", xs[..][5]);
}
