fn main() {
    // 1. Variables
    let mut num: u16 = 20; //immutable variable to make it mutable use mut keyword
    println!(" First Num is : {}", num);
    num = 200; //error: cannot assign twice to immutable variable
    println!(" Second Num is : {}", num);

    //2. Strings are of two types
    //1. String literals -> By default string literals are of type &str, string literals are immutable and fixed size stored in stack

    let string_litreal: &str = "Divyansh";
    println!("This is String Litreal : {}", string_litreal);

    //2. String -> String is a growable, mutable, owned, UTF-8 encoded string type, String is heap allocated
    let mut new_string: String = String::from("Divyansh is coding in Rust");
    new_string.push_str(" and learning it");
    println!("This is String: {}", new_string);

    //3. Tuples
    let hero: (&str, &str, u8) = ("Spiderman", "Peter Parker", 24);
    let (hero_name, real_name, hero_age) = hero;
    println!(
        "{} becomes {} at the age of {}",
        real_name, hero_name, hero_age
    );
}

//snake_case: all lowercase with underscores between words
//cargo: package manager and build system
//cargo build: compile the program
