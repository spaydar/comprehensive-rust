#![allow(dead_code)]

fn main() {
    arrays_and_for_loops_7_2();
}

fn hello_world() {
    println!("Hello 🌍!");
}

fn small_example_4_1() {
    let mut x: i32 = 6;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}

fn implicit_conversions_7_1() {
    fn multiply(x: i16, y: i16) -> i16 {
        x * y
    }

    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}

fn arrays_and_for_loops_7_2() {

    fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
        let mut transposed =  [[0i32; 3]; 3];
        for x in 0..3 {
            for y in 0..3 {
                transposed[x][y] = matrix[y][x];
            }
        }
        transposed
    }

    fn pretty_print(matrix: &[[i32; 3]; 3]) {
        println!("⎡{} {} {}⎤", matrix[0][0], matrix[0][1], matrix[0][2]);
        println!("⎟{} {} {}⎟", matrix[1][0], matrix[1][1], matrix[1][2]);
        println!("⎣{} {} {}⎦", matrix[2][0], matrix[2][1], matrix[2][2]);
    }

    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
