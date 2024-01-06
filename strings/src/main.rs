// Strings
//    1. String Literal (&str) - Primitive Type, Immutable, String value is known at compile time.
//    2. String Object (String) - Heap allocated Data Structure, Not null-terminated, Size can be modified.
// String Methods
//    1. str.capacity() - Gives capacity of string in bytes. Length of String will always be <= capacity.
//    2. str.contains("sub_str") - Find if one string contains another string.
//    3. str.replace(replace_from, replace_to) - Replace all occurance of substring with String Object.
//    4. string.trim() - Remove leading and trailing whitespaces in a string.
// Strings in Functions
//    1. &str - Passed as normal variable
//    2. String - Once passed in function argument, cannot be used in the calling function scope again.

fn strings() {
    // &str to String
    let lang = "Rust";
    let lang_obj = lang.to_string();
    println!("{}", lang_obj);

    // String
    let language = String::from("Rust Lang");
    println!("{}", language);
}

fn string_methods() {
    println!();
    let str = String::from("   Rust is a great new language is in the market for learning.");
    let str1 = "JavaScript";

    println!("{}", str.capacity());
    println!("{}", str.contains("Rust"));
    println!("{}", str.replace("Rust", str1));
    println!("{}", str.trim());
}

// String Iteration
fn str_iteration() {
    println!();
    let str1 = String::from("Rust is a great new language");
    let str2 = String::from("RUSTY");

    for i in str1.split(" ") {
        println!("{}", i);
    }

    for i in str2.chars() {
        println!("{}", i);
    }
}

// String Updation
fn str_updation() {
    println!();
    let mut str1 = String::from("Rus");
    str1.push('t');

    println!("{}", str1);

    str1.push_str(" Language");
    println!("{}", str1);

    let str2 = String::from(" Nice");
    let res = str1 + &str2;
    println!("{}", res);

    let str3 = String::from("Python");
    let format_macro = format!("{1} {0}", str2, str3);
    println!("{}", format_macro);

    let slice = &str3[0..2];
    println!("{}", slice);
}

fn main() {
    strings();
    string_methods();
    str_iteration();
    str_updation();
}
