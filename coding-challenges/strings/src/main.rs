// The task requires you to find all words starting with a “c” in a string passed as a parameter.
// Concatenate them together and return the result.
// Sample Input
//     "This is a comprehensive course in Rust programming language on the Educative. Read it with full concentration to grasp the content of the course"
// Sample Output
//     comprehensive course concentration content course

fn main() {
    let str = "This is a comprehensive course in Rust programming language on the Educative. Read it with full concentration to grasp the content of the course";
    let mut updated_str = "".to_string();
    
    for i in str.split(" ") {
        if i.starts_with('c') {
            updated_str.push_str(i);
            updated_str.push(' ');
        }
    }
    updated_str.pop();

    return updated_str;
}
