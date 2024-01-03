fn main() {
    // Local variable: Declared outside {}. Accessed inside that {} only.
    // Global variable: Declared inside {}. Accessed anywhere in the function.

    let global_variable = "global";
    {
        let local_variable = "local";
        println!("{}, {}", global_variable, local_variable);
    }

    // local_variable not accessible outside above scope {}.
    // println!("{}, {}", global_variable, local_variable);

    // Shadowing: Variable declared inside certain scope have same name as a variable declared outside scope.
    let _outer = 7;
    {
        let (_inner, _outer) = (2, 10);
        println!("{}, {}", _outer, _inner);
    }
    println!("{}", _outer);
}
