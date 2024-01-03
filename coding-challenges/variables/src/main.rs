// Problem Statement
//     Declare a variable x and store value 1000 in it.
//     Declare a variable y and store value “Programming” in it
//     Print the values of x and y
//     Change the value of x to 1100
//     Print the values of x and y
// Output
//     The output of the code should be:
//     x:1000
//     y:Programming
//     x:1100
//     y:Programming

fn main() {
    //! Using multiple variable binding
    let (mut x, y) = (1000, "Programming");
    println!("x:{}\ny:{}", x, y);
    x = 1100;
    println!("x:{}\ny:{}", x, y);
}
