fn main() {
    
    let mut var = 100;

    loop {
        var -= 1;
        if var % 13 == 0 {
            break;
        }
    }

    println!("The highest number under 100 divisible by 13 is {}.\n", var);
    
    let mut var = 0;
    let mut count = 0;

    let req_number = loop {
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("A number that is 3 and 5 divisible is {}.\n", var);
            count += 1;
            if count == 3 {
                break var;
            } else {
                continue;
            }
        }

        println!("{}", var);
    };

    println!("The highest number divisible between 3 and 5 is {}.", req_number);

}
