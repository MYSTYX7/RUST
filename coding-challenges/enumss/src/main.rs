// An enum Days has been provided to you. It has all the days of the week.
//     A method is_weekend() is incomplete.
//     The task is to complete the method
//     If the day is a weekend it returns 1
//     If the day is a weekday it returns 0
// Input
//   Day of the Week
// Output
//    1  or  0
// Sample Input
//    Wednesday
// Sample Output
//    0

#[allow(dead_code)]
#[derive(Debug)]
enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Days {
    fn is_weekend(&self) -> i32 {
        match self {
            Days::Saturday | Days::Sunday => return 1,
            _ => return 0,
        }
    }
}

fn main() {
    let day = Days::Saturday;
    println!("{:?}", day);
    println!("{}", day.is_weekend());
}
