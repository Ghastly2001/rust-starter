//snake_case: all lowercase with underscores between words
//cargo: package manager and build system
//cargo build: compile the program

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

    //4. Functions
    let num1: u8 = 10;
    let num2: u8 = 20;
    let avg: u8 = average(num1, num2);
    println!("The average of {} and {} is {}", num1, num2, avg);

    let name: &str = "Ghastly";
    let uppercase_name: String = uppercase_name(name);
    print!("Upppercase Name of {} is {}", name, uppercase_name);

    // 5. Ownership & Rules
    // 5.1. Each value in Rust has a variable that’s called its owner.
    // 5.2. There can only be one owner at a time.
    // 5.3. When the owner goes out of scope, the value will be dropped.

    let num1: u8 = 10;
    let num2: u8 = num1;

    println!("The value of num1 is: {} and num2 is {}", num1, num2); //num1 is copied to num2

    // let str1: String = String::from("Hello");
    // let str2 = str1;
    // println!("The value of str1 is: {} ans str2 is: {}", str1, str2); //will throw error
}

fn average(num1: u8, num2: u8) -> u8 {
    return (num1 + num2) / 2;
}

fn uppercase_name(name: &str) -> String {
    return name.to_uppercase();
}
