fn main() {

    println!("Enter a number:");
    let mut user_num = String::new();

    std::io::stdin()
    .read_line(&mut user_num)
    .expect("Failed to read input");
    
    let user_num: i32 = user_num.trim().parse().expect("Invalid integer");

    if user_num != 0 {
        if user_num % 2 == 0 {
            println!("The number is even.");
        } else {
            println!("The number is odd.");
        }
    } else {
        println!("The number is 0.");
    }

    let value: i32 = if true {
        1
    } else {
        2
    };

    println!("The value is {}", value);

    let other_results = 90;
    let grade = if other_results >= 90 {
        'A'
    } else if other_results >= 80 {
        'B'
    } else if other_results >= 70 {
        'C'
    } else if other_results >= 60 {
        'D'
    } else {
        'F'
    };

    println!("The grade is {}.", grade);

}
