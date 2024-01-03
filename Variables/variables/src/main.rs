fn main() {
    // Variable Binding - Variables are immutable by default
    let language = "Rust";
    println!("Language: {}", language);

    // Making variables mutable
    let mut language = "Rust";
    println!("Language: {}", language);
    language = "JavaScript";
    println!("Language: {}", language);

    // Multiple variables
    let (lang1, lang2) = ("Rust", "JavaScript");
    println!(
        "There are only 2 languages in the world!! {} and {}",
        lang1, lang2
    );
}
