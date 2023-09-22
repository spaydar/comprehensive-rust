#![allow(dead_code)]

fn main() {
    strings_and_iterators_22_2();
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

fn storing_books_19_1() {
    struct Library {
        books: Vec<Book>,
    }

    struct Book {
        title: String,
        year: u16,
    }

    impl Book {
        // This is a constructor, used below.
        fn new(title: &str, year: u16) -> Book {
            Book {
                title: String::from(title),
                year,
            }
        }
    }

    // Implement the methods below. Update the `self` parameter to
    // indicate the method's required level of ownership over the object:
    //
    // - `&self` for shared read-only access,
    // - `&mut self` for unique and mutable access,
    // - `self` for unique access by value.
    impl Library {
        fn new() -> Library {
            Library { books: vec![] }
        }

        fn len(&self) -> usize {
            self.books.len()
        }

        fn is_empty(&self) -> bool {
            self.books.is_empty()
        }

        fn add_book(&mut self, book: Book) {
            self.books.push(book);
        }

        fn print_books(&self) {
            self.books.iter().for_each(|b| println!("Title: {} | Year: {}", b.title, b.year));
        }

        fn oldest_book(&self) -> Option<&Book> {
            self.books.iter().min_by_key(|b| b.year)
        }
    }

    // This shows the desired behavior. Uncomment the code below and
    // implement the missing methods. You will need to update the
    // method signatures, including the "self" parameter! You may
    // also need to update the variable bindings within main.
    let mut library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());

    println!("Adding books...");
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());

    println!("The library has {} books", library.len());
    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }
}

// Health Statistics Exercise 19.2
fn health_statistics_19_2() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("bob.height() == 155.2: {}", bob.height() == 155.2);
}

pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        User { name, age, height, visit_count: 0usize, last_blood_pressure: None }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn doctor_visits(&self) -> u32 {
        self.visit_count as u32
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_height(&mut self, new_height: f32) {
        self.height = new_height;
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        let report = HealthReport {
            patient_name: self.name.as_str(),
            visit_count: self.doctor_visits() + 1,
            height_change: self.height() - measurements.height,
            blood_pressure_change: match self.last_blood_pressure {
                Some(bp) => Some((measurements.blood_pressure.0 as i32 - bp.0 as i32, measurements.blood_pressure.1 as i32 - bp.1 as i32)),
                None => None
            }
        };
        self.visit_count += 1;
        self.height = measurements.height;
        self.last_blood_pressure = Some(measurements.blood_pressure);
        report
    }
}

// Strings and Iterators 22.2
fn strings_and_iterators_22_2() {
    // println!("prefix_matches(\"/v1/publishers/*/books\",\"/v1/publishers/foo/books/book1\") {}",
    //          prefix_matches("/v1/publishers/*/books","/v1/publishers/foo/books/book1"));
    println!("/v1/publishers/*/books\", \"/v1/publishers/foo/booksByAuthor\") {}",
             prefix_matches("/v1/publishers/*/books","/v1/publishers/foo/booksByAuthor"));

}

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut prefix_tokens = prefix.split("/").skip(1);
    let mut request_path_tokens = request_path.split("/").skip(1);
    let mut wildcard_active = false;
    while let Some(p) = prefix_tokens.next() {
        if p == "*" {
            wildcard_active = true;
            continue;
        }
        if wildcard_active {
            while wildcard_active {
                match request_path_tokens.next() {
                    Some(r) => {
                        if r == p {
                            wildcard_active = false;
                        }
                    },
                    None => { return false; }
                }
            }
        } else {
            match request_path_tokens.next() {
                Some(r) if r == p => (),
                _ => { return false; }
            }
        };
    }
    true
}

// Tests
// Luhn tests
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

// Health Statistics Tests
#[test]
fn test_height() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.doctor_visits(), 0);
    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (120, 80),
    });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (115, 76),
    });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}

// Strings and Iterators Tests
#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
