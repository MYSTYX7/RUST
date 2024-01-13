// A struct Car and Motorbike is provided to you.
// A trait Drive is provided to you which has an abstract method can_drive.
// The task is to
//     - implement method can_drive for Car
//     - implement method can_drive for Motorbike
// Vehicle	Age Limit
//     Car ---- 18 or above
//     Motorbike ---- 14 or above
// Output
//     The output should be 0 or 1 based on whether a person can drive or not.

struct Car {
    owner_age: i32,
}

struct Bike {
    owner_age: i32,
}

trait Drive {
    fn can_drive(&self) -> i32;
}

impl Drive for Car {
    fn can_drive(&self) -> i32 {
        if self.owner_age >= 18 {
            return 1;
        } else {
            return 0;
        }
    }
}

impl Drive for Bike {
    fn can_drive(&self) -> i32 {
        if self.owner_age >= 14 {
            return 1;
        } else {
            return 0;
        }
    }
}

fn main() {
    let c = Car { owner_age: 16 };
    println!(
        "Can Age = {} drive the Car? : {}",
        c.owner_age,
        c.can_drive()
    );
    let b = Bike { owner_age: 17 };
    println!(
        "Can Age = {} drive the Bike? : {}",
        b.owner_age,
        b.can_drive()
    );
}
