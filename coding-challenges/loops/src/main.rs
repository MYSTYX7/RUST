// Task 1
// The task is to print the factorial of a number n.
// The factorial of a number n is the product of all numbers from 1 to n.
// Assume that the negative numbers may be passed in as argument and
//     that the factorial of a negative number should be set as zero.
// Sample Input
//   3
// Sample Output
//   6

fn factorial(num: i32) {
    let mut fact = 1;

    if num < 0 {
        println!("{}", 0);
    } else if num == 0 {
        println!("{}", 1);
    } else {
        for i in 1..=num {
            fact *= i;
        }
        println!("{}", fact)
    }
}

// Task 2
// A variable x is provided to you.
// Repeatedly decrease the value of the variable x by 3 each time,
//    as long as x is greater than or equal to 0.
// Print the number of times the iteration runs.
// Sample Input
//    21
// Sample Ouput
//    8

fn count_it(mut x: i32) {
    let mut count = 0;

    while x >= 0 {
        x -= 3;
        count += 1;
    }
    println!("{}", count);
}

// Write a code which prints a right-angled triangle of the character "&".
// The code takes an integer variable row as input and
// prints the right-angled triangle with that number of rows displaying "&".
// Sample Input
//    5
// Sample Output
//    &
//    &&
//    &&&
//    &&&&
//    &&&&&

fn tri(n: i32) {
    for i in 1..=n {
        for j in 1..=i {
            print!("&");
        }
        println!("");
    }
}

fn main() {
    factorial(4);
    count_it(21);
    tri(5);
}
