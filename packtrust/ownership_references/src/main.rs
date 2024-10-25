use std::vec;


fn stack_function(mut stack_num:i32) {
    stack_num = 56;
    println!("var = {}.", stack_num);
}

fn heap_function(var: &mut Vec<i32>) {
    var.push(50);
    println!("var: {:?}", var);
}

fn main() {
    let stack_num = 32;
    let mut heap_vec = vec![4, 5, 6];

    stack_function(stack_num);
    println!("The value inside of stack_num is: {}.", stack_num);

    heap_function(&mut heap_vec);
    println!("The value inside of heap_vec is: {:?}", heap_vec);

    let some_vec = vec![4, 5, 6];
    let ref1 = some_vec;
    let ref2 = &ref1;

    let data1 = String::from("Some long string");
    let data2 = String::from(" that continues");

    let concat_data = vec![&data1, &data2];
}
