fn main() {
    println!("Hello, world!");

    let x = 10.6;
    // Integers are implicitly cloned
    let y = x;

    println!("x: {}, y: {}", x, y);

    let string1 = String::from("Foo");
    // Strings are implicitly moved, so the first won't work
    // let string2: String = string1;
    // let string2: String = string1.clone();
    let string2: &String = &string1;
    // The & prefix creates a reference or pointer
    // Doesn't change owner at all

    println!("string1 = {}, string2 = {}", string1, string2);

    // This clone vs move concern largely concerns non-primitive variables

    let vector: Vec<i32> = vec![5,6,7,9,8,7];
    // let vectorAlt: &Vec<i32> = &vector
    let vector_alt: Vec<i32> = vector.clone();
    println!("vector: {:?}, vector_alt: {:?}", vector, vector);

    // {
    //     let my_name = String::from("Alan Dooley");
    // }

    // println!("My name is {}.", my_name);
}
