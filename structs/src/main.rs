// Struct - Consists of related items which might have different data types.
//        - Naming is done using CamelCase.
//        - Helps to create custom data type.
//        - Items are defined as "key:value pair".
//        - Keys = Items of Struct, Value = Data type of each item

struct Course {
    code: i32,
    name: String,
    level: String,
}

fn main() {
    let course1 = Course {
        code: 130,
        name: String::from("Rust"),
        level: String::from("Beginner"),
    };

    println!(
        "Name: {}, Level: {}, Code: {}",
        course1.name, course1.level, course1.code
    );

    let mut course2 = Course {
        code: 131,
        name: String::from("JavaScript"),
        level: String::from("Intermediate"),
    };
    println!("Before updating");
    println!(
        "Name: {}, Level: {}, Code: {}",
        course2.name, course2.level, course2.code
    );

    course2.code = 133;
    course2.level = String::from("Advanced");

    println!("After updating");
    println!(
        "Name: {}, Level: {}, Code: {}",
        course2.name, course2.level, course2.code
    );
}
