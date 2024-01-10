// Define a tuple struct
struct Fruits(String, i32);

fn main() {
    // Initialize tuple struct
    let r1 = Fruits("Mango".to_string(), 2);
    let r2 = Fruits("Apple".to_string(), 5);
    println!("{}, {}", r1.0, r1.1);
    println!("{}, {}", r2.0, r2.1);
}
