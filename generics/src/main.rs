// Generics = Define the data type at run time.
//          = Can be applied to methods, functions, enumerations, collections, and traits.
//          = Helps to reuse the same code with different type.
//          = <T> is known as Type Parameter. Used to declare generic construct.

fn main() {
    gen_fun();
    println!();
    gen_vec();
    println!();
    gen_struct();
}

// Generic Function
fn gen_fun() {
    println!("Generic Function");
    print!("- Passing String literal: ");
    concatenate("Rust", " Programming");
    print!("- Passing Integer: ");
    concatenate(6 as i32, 9 as i32);
}
use std::fmt::Display; // For printing a value of passed parameter
fn concatenate<T: Display>(t: T, s: T) {
    let res = format!("{}{}", t, s);
    println!("{}", res);
}

// Generic Vector
fn gen_vec() {
    println!("Generic Vector");
    print!("- Integer Vector: ");
    let int_vec: Vec<i32> = vec![5, 3, 7];
    print_vec(&int_vec);
    print!("- String Vector: ");
    let str_vec: Vec<&str> = vec!["Rust", "Javascript"];
    print_vec(&str_vec);
}
fn print_vec<T: Display>(v: &[T]) {
    for i in v.iter() {
        print!("{}", i);
    }
    println!("");
}

// Generic Struct
fn gen_struct() {
    println!("Generic Struct");
    print!("- Generic Type of i32: ");
    let r1 = Rectangle {
        width: 5,
        height: 6,
    };
    println!("Width= {}, Height= {}", r1.width, r1.height);
    print!("- Generic Type of f32: ");
    let r2 = Rectangle {
        width: 3.5,
        height: 8.8,
    };
    println!("Width= {}, Height= {}", r2.width, r2.height);
}
struct Rectangle<T> {
    width: T,
    height: T,
}
