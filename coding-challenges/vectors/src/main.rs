// Given a vector with an even number of elements, remove the last element from the input vector,
// and then the middle element.
// Then insert the sum of the remaining elements to the end of the resulting vector.
// Sample Input
//     [1, 5, 7, 9]
// Sample Output
//     [1, 7, 8]

fn test(vec: &mut Vec<u32>) -> &mut Vec<u32> {
    vec.pop();
    vec.remove(vec.len() / 2);

    let mut sum = 0;
    for i in vec.iter() {
        sum += i;
    }
    vec.push(sum);
    return vec;
}

fn main() {
    let mut v1 = vec![1, 5, 7, 9];
    println!("{:?}", test(&mut v1));
    let mut v2 = vec![1, 2, 3, 1, 2, 6];
}
