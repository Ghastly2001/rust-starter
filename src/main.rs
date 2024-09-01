use std::string;

fn main() {
    // Variables
    let mut num: u16 = 20; //immutable variable to make it mutable use mut keyword
    println!(" First Num is : {}", num);
    num = 200; //error: cannot assign twice to immutable variable
    println!(" Second Num is : {}", num);

    //Strings are of two types
    //1. String literals

    let string_litreal: &str = "Divyansh";
    println!("This is String Litreal : {}", string_litreal);
}

//snake_case: all lowercase with underscores between words
//cargo: package manager and build system
//cargo build: compile the program
