fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let pop_value = stack.pop();
    // println!("The popped value is {:?}.", pop_value);
    pop_value
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if stack.len() == maxsize {
        // println!("The stack is full.")
    } else {
        stack.push(item);
        // println!("Added {:?} to the stack.", item);
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("Failed to read input.");

    let n: u32 = n.trim().parse().expect("Invalid input");
    n 
}

fn main() {
    
    let input_string = String::from("Welcome to Rust.");
    let stack_size = input_string.len();
    let mut stack = new_stack(stack_size);
    let mut rev_string = String::new();

    for i in input_string.chars() {
        push(&mut stack , i, stack_size);
    }

    for i in 0..size(&stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("The original string is {:?}.", input_string);
    println!("The reverse is {:?}.", rev_string);

}