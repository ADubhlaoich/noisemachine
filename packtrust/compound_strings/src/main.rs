use std::fmt::format;

fn main() {
    let some_string = "Fixed length string";
    let mut growable_string = String::from("This string can change size");
    println!("The string is \"{}\".", growable_string);

    growable_string.push('E');
    println!("The string is \"{}\".", growable_string);

    growable_string.pop();
    println!("The string is \"{}\".", growable_string);

    growable_string.push_str(" at will.");
    println!("The string is \"{}\".", growable_string);

    println!("Basic string function,
    is_empty(): {},
    length: {},
    bytes: {},
    contains 'use': {}",
    growable_string.is_empty(), growable_string.len(), growable_string.capacity(), growable_string.contains("use"));

    growable_string.push_str("      ");
    let str_len = growable_string.trim().len();

    let number = 6;
    let num_str = number.to_string();
    println!("Is the variable number really a string? {}", number.to_string() == "6");

    let some_char = 'a';
    let char_str = some_char.to_string();

    let my_name = "Alan Dooley".to_string();

    let empty_string = String::new();
    println!("The length of empty_string is {}.\n", empty_string.len());

    let first = "Alan".to_string();
    let second = "Dooley".to_string();
    let hello = format!("My full name is {} {}", first, second);
    println!("{}", hello)

}
