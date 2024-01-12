// Traits = Define a standard set of behaviours for multiple structs.
// Types of traits
//    - Concrete Method = Implementation of method is done within the method.
//    - Abstract Method = Implementation of method is done by each struct it its own impl construct.

struct Circle {
    radius: f32,
}

struct Rectangle {
    width: f32,
    height: f32,
}

trait Area {
    fn area(&self) -> f32;
}

impl Area for Circle {
    fn area(&self) -> f32 {
        return 3.14 * self.radius * self.radius;
    }
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle {
        width: 2.5,
        height: 6.0,
    };

    println!("Area of Circle: {}", c.area());
    println!("Area of Rectangle: {}", r.area());
}
