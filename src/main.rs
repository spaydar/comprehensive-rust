#![allow(dead_code)]

fn main() {
    a_simple_gui_library_27_1();
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

// The solution seems to assume that a wildcard can only match one path segment.
// My implementation above works for cases where a wildcard can match an arbitrary
// number of path segments i.e. multiple directories
pub fn prefix_matches_solution(prefix: &str, request_path: &str) -> bool {

    let mut request_segments = request_path.split('/');

    for prefix_segment in prefix.split('/') {
        let Some(request_segment) = request_segments.next() else {
            return false;
        };
        if request_segment != prefix_segment && prefix_segment != "*" {
            return false;
        }
    }
    true
}

// A Simple GUI Library
pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.chars().count()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let _ = buffer.write_str(self.label.as_str());
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(format!("+{}+", "-".repeat(self.label.width() + 2)).as_str()).ok();
        buffer.write_str("\n| ").ok();
        self.label.draw_into(buffer);
        buffer.write_str(" |\n").ok();
        buffer.write_str(format!("+{}+", "-".repeat(self.label.width() + 2)).as_str()).ok();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        self.inner_width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.inner_width();
        buffer.write_str(format!("+{}+", "=".repeat(width + 2)).as_str()).ok();
        buffer.write_str(format!("\n| {: ^width$} |\n", self.title).as_str()).ok();
        buffer.write_str(format!("+{}+", "=".repeat(width + 2)).as_str()).ok();
        for w in self.widgets.iter() {
            let mut buf = String::new();
            w.draw_into(&mut buf);
            let mut lines = buf.lines();
            while let Some(line) = lines.next() {
                buffer.write_str(format!("\n| {: <width$} |", line).as_str()).ok();
            }
        }
        buffer.write_str(format!("\n+{}+", "-".repeat(width + 2)).as_str()).ok();
    }
}

fn a_simple_gui_library_27_1() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}

// Polygon Struct
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn magnitude(&self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }

    fn dist(&self, p2: Point) -> f64 {
        f64::from((self.x - p2.x).pow(2) + (self.y - p2.y).pow(2)).sqrt()
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Polygon {
        Polygon { points: vec![] }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p);
    }

    fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).cloned()
    }
}

impl std::ops::Deref for Polygon {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}

pub struct Circle {
    center: Point,
    radius: i32
}

impl Circle {
    fn new(p: Point, r: i32) -> Circle {
        Circle { center: p, radius: r}
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(p: Polygon) -> Self { Shape::Polygon(p) }
}

impl From<Circle> for Shape {
    fn from(c: Circle) -> Self { Shape::Circle(c) }
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Polygon(p) => {
                match p.iter().count() > 2 {
                    false => 0f64,
                    true => {
                        let mut prev = None;
                        let mut d = p.points.first().unwrap().dist(*p.points.last().unwrap());
                        for p in p.iter() {
                            if let Some(pl) = prev {
                                d += p.dist(pl);
                            }
                            prev = Some(*p);
                        }
                        d
                    }
                }
            },
            Shape::Circle(c) => {
                c.radius as f64 * 2f64 * std::f64::consts::PI
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

// Safe FFI Wrapper
fn safe_ffi_wrapper_31_1() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_ulong, c_ushort, c_uchar};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout according to the Linux man page for readdir(3), where ino_t and
    // off_t are resolved according to the definitions in
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // Layout according to the macOS man page for dir(5).
    #[cfg(all(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        // See https://github.com/rust-lang/libc/issues/414 and the section on
        // _DARWIN_FEATURE_64_BIT_INODE in the macOS man page for stat(2).
        //
        // "Platforms that existed before these updates were available" refers
        // to macOS (as opposed to iOS / wearOS / etc.) on Intel and PowerPC.
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        unimplemented!()
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        unimplemented!()
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        unimplemented!()
    }
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
