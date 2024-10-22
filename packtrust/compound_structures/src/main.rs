use core::num;

fn main() {
    
    let my_information = ("Salary", 60000);
    println!("{} and {} exist within the tuple.", my_information.0, my_information.1);
    println!("This is the same thing but at once: {:?}", my_information);

    let(salary, salary_value) = my_information; 
    let salary = my_information.0;
    let salary_value = my_information.1;

    let nested_value= (4, 5.0, (3,2), "Hello");
    let element = nested_value.2.0;

    let empty_tuple = ();

    let mut number_array = [4, 5, 6, 8, 9];

    println!("Here are the values: {} {} {} {} {}", number_array[0], number_array[1], number_array[2], number_array[3], number_array[4]);
    println!("And the array itself: {:?}", number_array);

    let array_with_same_elements = [0; 10];

    let mut array_with_strings =  ["apple", "tomato", "grapes"];
    let array_with_strings_2 = ["Unknow";6];

    let char_array = ['a', 'b', 'c'];

    let mut number_array = [4, 5 , 6, 8, 9];
    let number_subset = &number_array[0..3];
    println!("{:?}", number_subset);
}
