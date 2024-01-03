// Problem Statement 1
//     Declare an array, arr, of size 6 that has numbers divisible by 2 ranging from 0 to 10.
//     Print the value of arr
// Output
//    0, 2, 4, 6, 8, 10

// Problem Statement 2
//     Declare a tuple, persons, of size 6 that has the names of people along with their ages
//     Print the value of persons
// Output
//     Alex:21, Abe:22, Anna:23

fn main() {
    arr();
    tuple();
}

fn arr() {
    let even: [i8; 6] = [0, 2, 4, 6, 8, 10];
    println!(
        "{},{},{},{},{},{}",
        even[0], even[1], even[2], even[3], even[4], even[5]
    );
}

fn tuple() {
    let persons = ("Alex", 21, "Abe", 22, "Anna", 23);
    println!(
        "{}:{}, {}:{}, {}:{}",
        persons.0, persons.1, persons.2, persons.3, persons.4, persons.5
    );
}
