fn check_order(tuple: (i32, i32, i32)) -> bool{
    let (left, middle, right) = tuple;
    left < middle && middle < right
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3]{
    let mut result: [[i32; 3]; 3] = [[0;3];3];
    for i in 0..3 {
        for j in 0..3{
            result[i][j] = matrix[j][i]
        }
    }
    return result;
}

fn main() {
    // Arrays
    // [T;n] T type of element in the array and n is the number of element.
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:?}");
    let a: [f32; 24] = [0.0; 24];
    println!("a: {a:?}");

    // Tuples
    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);


    // Array iteration
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes{
        for i in 2..prime{
            assert_ne!(prime % i, 0);
        }
    }

    let tuple = (1, 5, 3);
    println!(
        "{tuple:?}: {}",
        if check_order(tuple) { "ordered" } else { "unordered" }
    );

    //exercise
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
}
