// Definite Loop
//     1. For
// Indefinite Loop
//     1. While
//     2. Loop

fn for_loop() {
    println!("For loop");
    // starting range is inclusive, ending range is exclusive
    for i in 0..5 {
        println!("{}", i)
    }

    // .enumerate() counts the no. of times loop has executed
    for (count, variable) in (7..10).enumerate() {
        println!("count = {}, variable = {}", count, variable);
    }
}

fn while_loop() {
    println!("While loop");
    let mut x = 6;

    while x > 5 {
        x += 1;
        println!("{}", x);
        if x == 10 {
            break;
        }
    }
}

fn loop_loop() {
    println!("Loop loop");

    let mut x = 1;
    loop {
        x = x + 1;
        println!("{}", x);
    }
}

fn main() {
    for_loop();
    while_loop();
    loop_loop();
}
