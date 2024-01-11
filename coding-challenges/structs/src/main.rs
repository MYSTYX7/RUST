// A struct Point is given which has two items, x and y.
// The function test is given which has two instances of points initialized with some value of x and y.
// The task is to calculate the distance between the two points.
// The distance between two points is: sqrt((x1-x2)^2 + (y1-y2)^2)
// Input
//     Two instances of point
// Output
//     The output of the code should be: The distance between x and y

struct Point {
    x: i32,
    y: i32,
}

fn test_struct(p1: Point, p2: Point) -> f32 {
    let dis = i32::pow(p1.x - p2.x, 2) + i32::pow(p1.y - p2.y, 2);
    let distance = dis as f32;
    return distance.sqrt();
}

fn main() {
    let point1 = Point { x: 3, y: 4 };
    let point2 = Point { x: 12, y: 13 };

    println!("Distance is {}", test_struct(point1, point2));
}
