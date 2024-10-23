use std::vec;

fn main() {
    let mut number_vector: Vec<i32> = vec![4, 5, 6 ,8 , 9, 10, 11, 12, 15, 17, 12, 10];

    println!("{}", number_vector[0]);
    println!("{:?}", number_vector);
    
    number_vector[4] = 5;
    println!("{:?}", number_vector);

    let vector_zero_array: Vec<i32> = vec![0;10];
    println!("{:?}", vector_zero_array);

    let vector_string_array: Vec<&str> = vec!["apple", "orange", "banana"];
    println!("{:?}", vector_string_array);
}
