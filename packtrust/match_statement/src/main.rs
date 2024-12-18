fn main() {
    
    let some_number = 100;

    match some_number {
        1 => println!("The number is 1"),
        2 | 3 => println!("The number is 2 or 3"),
        4..=100 => println!("The number is between 4 and 100."),
        _ => println!("The number is 0 or greater than 100.")
    }

    let marks = 50;
    let mut grade = 'N';

    match marks {
        90..=100 => grade = 'A',
        80..=89 =>  grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F'
    }

    println!("The grade is {}.", grade);

    let marks = 85;
    let grade = match marks {
        90..=100 => 'A',
        80..=89 =>  'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F'
    };

    println!("The grade is {}.", grade);
}

