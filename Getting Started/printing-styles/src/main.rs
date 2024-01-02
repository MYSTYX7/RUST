fn main() {
    // print!() : Prints in same line
    print!("1. Hello same line");
    print!("2. Hello same line");

    // println!() : Appends new line character at the end of string
    println!("");
    println!("Hey Below line");
    println!("Hey Above line");

    // eprint!() : Prints ths string as error
    eprint!("Error: This is error 1");
    eprint!("Error: This is error 2");

    // eprintln!() : Prints ths string as error
    println!("");
    eprintln!("Error: This is new line error 1");
    eprintln!("Error: This is new line error 2");
}
