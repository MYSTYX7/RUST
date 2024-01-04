// Conditional Expressions
//     1. if
//     2. if let - Conditional Expression that allows pattern matching
//     3. match - Checks if current values corresponds to any value in the list of values.
//              - Similar to Switch statement.

fn if_exp() {
    let lang1 = "Rust";

    // Method 1
    if lang1 == "Rust" {
        println!("RUST Lang");
    } else {
        println!("JavaScript");
    }

    // Method 2: Shorthand syntax
    let res = if lang1 == "Rust" {
        "RUST Lang"
    } else {
        "JavaScript"
    };
    println!("{}", res);

    let lang2 = "Python";
    if lang1 == "Rust" && lang2 == "Python" {
        println!("{} & {}", lang1, lang2);
    } else {
        println!("Wrong");
    }
}

fn if_let() {
    let course = ("Rust", "TypeScript", "Java");

    if let ("Rust", a, b) = course {
        println!("After Rust, 2 languages are: {} & {}", a, b);
    }
}

fn match_exp() {
    let (x, y) = (3, 7);

    // Method 1
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else"),
    };

    // Method 2
    let res = match y {
        1 => 11,
        2 => 22,
        3 => 33,
        _ => 100,
    };

    println!("Res:{}", res);
}

fn main() {
    if_exp();
    if_let();
    match_exp();
}
