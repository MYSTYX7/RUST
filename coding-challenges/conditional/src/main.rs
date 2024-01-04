// Task 1
// Write a code which will check whether _a given integer number is even or odd.
// If the number is even print “Number a is even” and if it’s an odd print “Number a is odd”.
// Sample Input
//    3
// Output
//    Number 3 is odd

// Task 2
// Write a code which will take:
// - Two variables named a and b
// - a character type variable called operator which will take operators (+,-,/,*,%)
//     will be passed as input to our match statement
// - Use match statements to compute: addition of a and b, subtraction of b from a,
//     multiplication of a and b, division of a by b, modulus of a by b
// Sample Input
// Suppose three inputs are given:
//     a = 3,  b = 2,  operator = '+'
//     a = 3 , b = 2,  operator = '('
//     a = 3 , b = 0,  operator = '/'
// Sample Output
// The output of the program corresponding to input:
//     5
//     invalid operator
//     Division by 0 is undefined

fn task1(_a: i32) {
    if _a % 2 == 0 {
        println!("Number {} is even", _a);
    } else {
        println!("Number {} is odd", _a);
    }
}

fn task2(a: i32, operator: char, b: i32) {
    match operator {
        '+' => println!("{}", a + b),
        '-' => println!("{}", a - b),
        '*' => println!("{}", a * b),
        '/' => {
            if b == 0 {
                println!("Division by 0 is undefined");
            } else {
                println!("{}", a / b);
            }
        }
        '%' => {
            if b == 0 {
                println!("Mod 0 is undefined");
            } else {
                println!("{}", a % b);
            }
        }
        _ => println!("invalid operator"),
    }
}

fn main() {
    task1(3);

    print!("3 + 2: ",);
    task2(3, '+', 2);
    print!("3 - 2: ");
    task2(3, '-', 2);
    print!("3 * 2: ");
    task2(3, '*', 2);
    print!("3 / 2: ");
    task2(3, '/', 2);
    print!("3 % 2: ");
    task2(3, '%', 2);
    print!("3 ( 2: ");
    task2(3, '(', 2);
    print!("3 / 0: ");
    task2(3, '/', 0)
}
