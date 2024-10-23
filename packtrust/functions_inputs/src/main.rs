fn main() {
    basic_fn();
    function_with_inputs("Alan Dooley", 60_000);

    let full_name = "Some name";
    let full_salary = 35_000;

    function_with_inputs(full_name, full_salary);

    println!("{} multiplied by {} is {}.", 50, 20, function_nums_output(50, 20));
    // println!("The same numbers, multiplied: {}, added: {}, subtracted: {}", function_multi_output(50, 20));
    // Store the result in a tuple variable, then try this again using dot notation

    let full_name: String = {
        let first_name = "Alan";
        let surname = "Dooley";
        format!("{} {}", first_name, surname)
    };

    println!("My full name is {}.", full_name);

    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("Failed to read input.");

    let n: f64 = n.trim().parse().expect("Invalid input");

    println!("{:?}", n);

}

fn basic_fn() {
    println!("Hello, world!");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("{} has a salary of {}.", name, salary);
}

fn function_nums_output(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn function_multi_output(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1+num2, num1-num2)
}