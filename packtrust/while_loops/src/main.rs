fn main() {

    // loop {
    //     println!("This is an infinite loop.");
    // }

    // let my_number = 5;
    // let mut guess = false;

    // println!("Guess a number between 1 and 20:");

    // while guess != true {
    //     let mut number = String::new();
    //     std::io::stdin()
    //     .read_line(&mut number)
    //     .expect("Failed to read input");

    //     let number: u8 = number.trim().parse().expect("Invalid input");

    //     if number == my_number {
    //         println!("You've guessed the correct number!");
    //         guess = true;
    //     } else {
    //         println!("Wrong number: try again.");
    //     }
    // }

    println!("Enter a number: I will give you the next number divisible by 2 and 5.");

    let mut number = String::new();

    std::io::stdin()
    .read_line(&mut number)
    .expect("Failed to read input.");

    let mut number: u8 = number.trim().parse().expect("Invalid input");
    number += 1;
    while (number % 2 == 0 && number % 5 == 0) != true {
        number += 1;
    }
    print!("The number after yours divisible by 2 and 5 is {}", number);

}
