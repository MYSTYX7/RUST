// Vectors = Resizable Array
// .iter() = Iterate over Vector to find the index of particulr element
// .iter_mut() = Values of vector within the loop can be changed
// Slicing a Vector = Two word object. First one is pointer to data, Second is length of the slice

fn vectors() {
    let mut vec = vec![1, 2, 3];

    println!("{}", vec[2]);
    println!("{:?}", vec);
    println!("{}", vec.contains(&2));

    vec.push(4);
    vec.push(5);
    vec.push(6);
    println!("{:?}", vec);
    println!("Capacity:{}  Length:{}", vec.capacity(), vec.len());

    vec.pop();
    vec.remove(1);
    println!("{:?}", vec);
}

fn iterating() {
    println!();
    let mut vec = vec![5, 7, 8, 4, 3];
    let mut index = vec.iter().position(|&e| e == 4).unwrap();
    vec.remove(index);
    println!("{:?}", vec);

    index = 0;
    for i in vec.iter() {
        println!("Index:{}, Element:{}", index, i);
        index += 1;
    }
}

fn mut_vec() {
    println!();
    let mut vec = vec![28, 634, 376, 82];
    for i in vec.iter_mut() {
        *i /= 2;
    }
    println!("{:?}", vec);
}

fn vec_slicing() {
    println!();
    let vec = vec![5, 7, 8, 4, 3];
    let sliced = &vec[1..4];
    println!("Sliced: {:?}", sliced);
}

fn main() {
    vectors();
    iterating();
    mut_vec();
    vec_slicing();
}
