// Pass by value
//     Parameters value changed inside called function's arguments won't be changed at calling fucntion's lcoation.
// Pass by reference
//     Parameters value changed in called function will be changed at calling function location.

fn pass_by_value(mut arr1: [i32; 5]) -> [i32; 5] {
    arr1[2] = 7;
    println!("Inside Pass by Value: {:?}", arr1);
    return arr1;
}

fn pass_by_reference(arr2: &mut [i32; 4]) -> [i32; 4] {
    *arr2 = [11 + 1, 12 + 2, 13 + 3, 14 + 4];
    println!("Inside Pass by Reference: {:?}", arr2);
    return *arr2;
}

fn main() {
    let (arr1, mut arr2) = ([1, 2, 3, 4, 5], [11, 12, 13, 14]);

    println!("Outside Pass by Value: {:?}", pass_by_value(arr1));
    println!(
        "Outside Pass by Reference: {:?}",
        pass_by_reference(&mut arr2)
    );
}
