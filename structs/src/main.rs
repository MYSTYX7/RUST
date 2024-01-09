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
    print_struct(course1);

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

    let course3 = Course {
        code: 135,
        name: String::from("Python"),
        level: String::from("Advanced"),
    };

    let ret = return_struct(course2, course3);
    println!("Values from return_struct function");
    println!("Name:{}, Leve:{}, Code:{}", ret.name, ret.level, ret.code);
}

fn print_struct(c: Course) {
    println!("Printing from print_struct function");
    println!("Name: {}, Level: {}, Code: {}", c.name, c.level, c.code);
}

fn return_struct(c2: Course, c3: Course) -> Course {
    if c2.name == "JavaScript" {
        return c2;
    } else {
        return c3;
    }
}
