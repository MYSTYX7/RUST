fn main() {
    // Single Placeholder: Used to print a single value
    println!("Number: {}", 7);

    // Multiple Placeholder
    println!("My {} is {}", "name", "Abhinav");

    // Positional Arguments
    println!("Recently got to know about {0}. Learning this new language called {0} is fun. I have the most experience in {1} language.", "Rust", "JavaScript");

    // Named Arguments
    println!(
        "I'm switching my main programming language from {old} to {new}.",
        new = "Rust",
        old = "JavaScript"
    );

    // Placeholder Traits
    println!(
        "Number: 10 --- Binary: {:b} --- Hexa-decimal: {:x} --- Octal: {:o}",
        10, 10, 10
    );

    // Maths
    println!("{} * {} = {}", 10, 20, 10 * 20);

    // Debut Trait Placeholder
    println!("{:?}", ("Rust is Awesome", 10));
}
