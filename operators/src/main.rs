// Type Casting: Converting one variable's data type to another data type
//    Typecasting is done using the `as` keyword.

fn typecast() {
    let a = 15;
    let b = (a as f64) / 2.0;

    println!("{}\n", b);
}

// Borrowing and Derefencing
// Borrowing means to reference the original data binding or to share the data.
//    1. Shared Borrowing => Can not be altered, Syntax is `let operand1 = & operand2`
//    2. Mutable Borrowing => Can be altered, Syntax is `let operand1 = & mut operand2`
// Derefrencing refers to changing the value of the referenced variable using its address stored
// in the referencing variable. Syntax is `*operand1 = operand2`
fn bor() {
    let (x, mut y) = (10, 13);

    let a = &x;
    println!("a:{}\nx:{}", a, x);

    let b = &mut y;
    println!("b:{}", b);

    *b = 7;
    println!("b:{}", b);
    println!("y:{}", y);
}

fn main() {
    typecast();
    bor();
}
