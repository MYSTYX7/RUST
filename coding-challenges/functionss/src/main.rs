// Task 1
// Write a function test_divisibility_by_3_4 which will check whether a given integer number is divisible by 3 or 4.

// - If the number is divisible by both return 0
// - If the number is divisible by 3 only return 1
// - If the number is divisible by 4 only return 2
// - If the number is not divisible by both, return -1

// Input
//     integer
// Output
// The output can be any of the following:

//     0 , 1 , 2 ,-1
// Sample Input
//     12 , 9 , 16 , 19
// Sample Output
//     0 , 1 , 2 , -1

fn task1(a: i16) -> i16 {
    if a % 3 == 0 && a % 4 == 0 {
        0
    } else if a % 3 == 0 && a % 4 != 0 {
        1
    } else if a % 3 != 0 && a % 4 == 0 {
        2
    } else {
        -1
    }
}

// Task 2
// The task is to modify the given function arr_square() to use loops to iterate over an array,
//     which is declared within the function, and replace all of its elements by their squares.
// Test Sample
//     An array of integers declared in the funtion: [1, 2, 3, 4, 5]
// Output
//     The function should return the following array: [1, 4, 9, 16, 25]

fn task2() -> [i32; 5] {
    // Write code here!
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        arr[i] = arr[i] * arr[i];
    }
    return arr;
}

// Task 3
// In this exercise, you have to write a recursive function fibonacci
//     that takes a positive integer number n as a parameter and
//     returns the nth Fibonacci term in that range.
// Sample Input
//   7
// Sample Output
//   13

fn task3(term: i16) -> i16 {
    if term == 0 {
        return 0;
    } else if term == 1 {
        return 1;
    } else {
        return task3(term - 1) + task3(term - 2);
    }
}

fn main() {
    println!("{}", task1(12));
    println!("{:?}", task2());
    println!("{}", task3(7));
}
