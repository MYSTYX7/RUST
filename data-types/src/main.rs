// Primitive Data Types
// 1. Scalar
//     a. Numeric
//         i. Integer
//         ii. Float
//     b. Non-Numeric
//         i. Boolean
//         ii. Character & String
// 2. Compound
//     a. Array
//     b. Tuple

// Integers
// - Signed: i8, i16, i32, i64        i8 = -2^8 to (2^8 - 1)
// - Unsigned: u8, u16, u32, u64      u16 = 0 to (2^16 - 1)
fn int() {
    let a: u8 = 255;
    println!("a:{}", a);
}

// Float
// - 32bit
// - 64bit
fn flt() {
    let b: f32 = -2148.5;
    println!("b:{}", b);
}

// Boolean
// - bool
fn bool() {
    let c: bool = true;
    let d = 10 > 20;
    println!("c:{}, d:{}", c, d);
}

// Character & String
// char: Used to store single character value. Single quotes are used for char ('').
// &str: Used to store string value. Double quotes are used for string ("").
fn chars() {
    let (e, f) = ('z', "xyz");
    println!("e:{}, f:{}", e, f);
}

// Array
// Collection of values of same type stored in single variable
// .len(): to get size of array
// Slice: Portion of an array. Starting index in inclusive, Ending index is exlclusive.
fn arr() {
    let new_arr: [&str; 4] = ["Abhi", "Singh", "Chauhan", "MYSTYX7"];
    println!("First element is {}", new_arr[0]);
    println!("All elements using Debug Trait {:?}", new_arr);

    let arr1 = [7; 4];
    println!(
        "Array of length {} with all elements {}",
        arr1.len(),
        arr1[0]
    );

    let slice_arr = &new_arr[3..4];
    println!("{:?}", slice_arr);
}

// Tuple
// Collection of values of different types
//
fn tuple() {
    let person_data = ("Alex", 48, "35kg", "6ft");
    println!("Name:{}", person_data.1);

    // Destructuring
    let (w, x, y, z) = person_data;
    println!("Name:{}, Age:{}, Weight:{}, Height:{}", w, x, y, z);

    println!("Person Data: {:?}", person_data);
}

// Constant Variables
// Immutable, Mandatory to define data type, Cannot be shadowed, Written with SCREAMING_SNAKE_CASE

fn main() {
    int();
    flt();
    bool();
    chars();
    arr();
    tuple();
}
