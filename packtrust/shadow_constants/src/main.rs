fn main() {
    let (first_number, second_number) = (250, 480.22);
    let large_number = 1_000_000;
    // let overflow_number: u8 = 256;
    let x = 255;
    println!("The value of x in octal is {:o}, in hexidecimal {:X}, and {:b} in binary", x, x, x);

    let number = 45;

    let n1 = 14;
    let n2 = 15.6;

    let n3 = n1 as f64 + n2;
    println!("n1 is {}.", n3);

    let s = 5;
    let s = 5*5;

    println!("s is {}.", s);

    let mut p = 5;
    let p = 5*5;

    let q = 32;
    let q= 'A';

    let mut r = 65;
    { 
        let r = 60;
        println!("r is {} within this code segment.", r);
    }
    println!("r is {} outside the code segment.", r);

const SOMETHING_AXIOMATIC: u32 = 1337;

}
