use std::result::IterMut;

fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let pop_val: Option<String> = stack.pop();
    pop_val
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
    } else {
        stack.push(item);
    }
}

fn size(stack: &Vec<String>) -> usize {
    stack.len()
}

fn individual_symbols(input_expr: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();
    let input_chars: Vec<char> = input_expr.chars().collect();
    let mut temp: Vec<char> = Vec::new();
    for i in input_chars {
        if i != '+' && i != '-' && i != '/' && i != '*' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(i.to_string());
            } else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![];
            }
        }
    }
    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }
    println!("{:?}", tokenized_input);
    tokenized_input
}

fn main() {
    let input_expr: String = String::from("(33+45/3*(2+9)-50)");
    println!("The input express is {:?}", input_expr);
    let input_expr_tokenized = individual_symbols(input_expr);
}
