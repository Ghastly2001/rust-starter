fn main() {
    // Variables
    let mut num: u16 = 20; //immutable variable to make it mutable use mut keyword
    println!(" First Num is : {}", num);
    num = 200; //error: cannot assign twice to immutable variable
    println!(" Second Num is : {}", num);

    //Strings are of two types
    //1. String literals -> By default string literals are of type &str

    let string_litreal: &str = "Divyansh";
    println!("This is String Litreal : {}", string_litreal);

    let new_string = String::from("Divyansh is coding in Rust");
    println!("This is String: {}", new_string);
}

//snake_case: all lowercase with underscores between words
//cargo: package manager and build system
//cargo build: compile the program
