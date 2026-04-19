// ============================================================
// GENERICS IN RUST — Learning File
// Run individual sections by calling them from main.rs
// ============================================================

// --- 1. Generic Functions ---
// Instead of writing add_i32, add_f64, etc., one function handles all numeric types.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// --- 2. Generic Structs ---
// A Point that can hold any type for x and y.
struct Point<T> {
    x: T,
    y: T,
}

// --- 3. Methods on Generic Structs ---
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Concrete impl only for Point<f64>
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// --- 4. Multiple Generic Type Params ---
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Self { first, second }
    }
}

// --- 5. Generic Enums (like Option and Result in stdlib) ---
// This is exactly how Option<T> works under the hood:
enum MyOption<T> {
    Some(T),
    None,
}

// --- 6. Combining Generics with Traits (Trait Bounds) ---
// T must implement Display so we can print it.
use std::fmt::Display;

fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    let l = largest(list);
    println!("The largest is: {}", l);
}

// Where clause syntax (cleaner for complex bounds):
fn compare_and_display<T, U>(t: &T, u: &U)
where
    T: Display + PartialOrd,
    U: Display,
{
    println!("Comparing {} and {}", t, u);
}

// --- YOUR PLAYGROUND ---
// Add your own generic types and functions below and call
// them from run_generics_examples().

pub fn run_generics_examples() {
    // Generic function
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    // Generic struct
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0_f64, y: 4.0_f64 };
    println!("int_point.x = {}", int_point.x());
    println!("distance from origin: {:.2}", float_point.distance_from_origin());

    // Multiple type params
    let pair = Pair::new("hello", 42);
    println!("Pair: ({}, {})", pair.first, pair.second);

    // Generic enum
    let some_val: MyOption<i32> = MyOption::Some(7);
    match some_val {
        MyOption::Some(v) => println!("Got value: {}", v),
        MyOption::None => println!("Nothing"),
    }

    // Trait bounds
    print_largest(&numbers);
    compare_and_display(&"rust", &3.14_f64);
}
