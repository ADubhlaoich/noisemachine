fn main() {
    let some_number = 55;

    if some_number < 50 {
        println!("The number is less than 50!");
    }

    println!("This will print no matter what.");

    let marks = 65;

    if marks > 60 && marks <= 70 {
        println!("The marks are between 60 and 70.");
    }

    let flag_1 = true;
    let flag_2 = false;

    if flag_1 == true || flag_2 == true {
        println!("At least one condition is true.");
    }

    let flag_3 = true;
    if flag_3 != false {
        println!("This displays if the condition is true or not false.");
    }

    let flag_4 = true;
    let flag_5 = false;
    let other_number = 60;

    if (flag_4 == true && flag_5 == false) || other_number < 50 {
        println!("At least one condition is true, again");
    }

    let results = 80;
    if results > 50 {
        println!("You've passed.");
    } else {
        println!("You've failed...");
    }

    let other_results = 90;
    let mut grade = 'N';
    if other_results >= 90 {
        grade = 'A';
    } else if other_results >= 80 {
        grade = 'B';
    } else if other_results >= 70 {
        grade = 'C';
    } else if other_results >= 60 {
        grade = 'D';
    } else {
        grade = 'F'
    }

    print!("The grade obtained is {}.", grade);
}
