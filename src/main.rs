#![allow(dead_code)]

fn main() {
    luhn_algorithm_13_1();
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

fn luhn_algorithm_13_1() {
    println!("{}", luhn("4263 9826 4026 9299"));
}

fn luhn(cc_number: &str) -> bool {
    fn iteratively(cc_number: &str) -> bool {
        let mut removed_whitespace = vec![];
        for c in cc_number.chars() {
            if !c.is_whitespace() {
                removed_whitespace.push(c);
            }
        }
        let mut digits = vec![];
        for c in removed_whitespace {
            if !c.is_ascii_digit() {
                return false;
            } else {
                digits.push(c.to_digit(10).unwrap());
            }
        }
        if digits.len() < 2 {
            return false;
        }
        for i in (0..digits.len()).rev().skip(1).step_by(2) {
            let mut doubled = digits[i] * 2;
            let mut result = 0;
            while doubled > 0 {
                result += doubled % 10;
                doubled /= 10;
            }
            digits[i] = result;
        }
        digits.into_iter().sum::<u32>() % 10 == 0
    }
    fn solution(cc_number: &str) -> bool {
        let mut digits_seen = 0;
        let mut sum = 0;
        for (i, ch) in cc_number.chars().rev().filter(|&ch| ch != ' ').enumerate() {
            match ch.to_digit(10) {
                Some(d) => {
                    sum += if i % 2 == 1 {
                        let dd = d * 2;
                        dd / 10 + dd % 10
                    } else {
                        d
                    };
                    digits_seen += 1;
                }
                None => return false,
            }
        }

        if digits_seen < 2 {
            return false;
        }

        sum % 10 == 0
    }
    iteratively(cc_number)
}

// Tests
#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
