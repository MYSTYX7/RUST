// Modules - Collection of items that can contain structs, functions, enums, vectors, arrays, etc.
// Privacy Rules
//    - Rule 1 = If an item is public it can be accessed anywhere.
//    - Rule 2 = If an item is private it can be accessed through:
//            - Parent module using the private function indirectly through public function.
//            - Private function through Child Module.

mod r1 {
    pub fn print_statement() {
        println!("This is Module Print");
    }
}

mod r2_1 {
    fn private_func() {
        println!("Inside Private Function");
    }

    pub fn public_func() {
        println!("Inside Public Function");
        self::private_func();
    }
}

mod outer_module {
    fn my_pri_func() {
        println!("Outer Module - Private Function");
    }

    pub mod inner_module {
        pub fn my_pub_func() {
            println!("Inner Module - Public Function");
            super::my_pri_func();
        }
    }
}

fn main() {
    println!("- Rule 1: ");
    r1::print_statement();
    println!("");
    println!("- Rule 2.1: ");
    r2_1::public_func();
    println!("");
    println!("- Rule 2.2: ");
    outer_module::inner_module::my_pub_func();
}
