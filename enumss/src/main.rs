// Enums = Custom data type that is composed of Variants(= Values which are definite)

#[derive(Debug)]
enum KnightMoves {
    Horizontal(String),
    Vertical(String),
}

//Enum Methods
#[allow(dead_code)]
#[derive(Debug)]
enum TrafficLights {
    Red,
    Yellow,
    Green,
}

impl TrafficLights {
    fn is_stop(&self) -> bool {
        match self {
            TrafficLights::Red => return true,
            _ => return false,
        }
    }
}

// Option = Option is built-in enum in Rust. Have two variants "Some(data type)" and "None".
fn learn_lang(my_lang: String) -> Option<bool> {
    if my_lang == "Rust" {
        return Some(true);
    } else {
        return None;
    }
}

// Result = Result is built-in enum in Rust. Have two variants "Ok(T)" and "Err(E)".
fn file_found(i: bool) -> Result<i32, bool> {
    if i {
        Ok(200)
    } else {
        Err(false)
    }
}

fn main() {
    let h_move = KnightMoves::Horizontal;
    let v_move = KnightMoves::Vertical;
    println!("Move 1: {:?}", h_move("Left".to_string()));
    println!("Move 2: {:?}", v_move("Down".to_string()));

    let action = TrafficLights::Red;
    println!("What is the signal value? : {:?}", action);
    println!("Do we have to stop? : {}", action.is_stop());

    let lang = learn_lang(String::from("Rust"));
    println!("Language result is {:?}", lang);

    println!("{:?}", file_found(true));
}
