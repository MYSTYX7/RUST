// The task is to compute (a+b)^3 and display it on the console.
// The following mathematical formula is expanded as: (a+b)^3=a^3+b^3+3ab(a+b)
// Sample Input
//     a = 2
//     b = 2
// Output
//     64

fn main() {
    let (a, b) = (2, 2);
    
    let res = i8::pow(a, 3) + i8::pow(b, 3) + (3 * a * b * (a + b));

    println!("{}", res);
}
