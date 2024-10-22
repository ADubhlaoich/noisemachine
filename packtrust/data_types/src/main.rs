fn main() {
    let x: f32 = 15.0;
    let y = 10;
    let mut z = 3;

    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);
    println!("The value of z is {}.", z);
    z = 19;
    println!("The new value iz s is {}.", z);

    println!("The maximum value of a i8 signed integer is: {}.", std::i8::MAX);
    println!("The maximum value of a u8 signed integer is: {}.", std::u8::MAX);

    let status = false;
    println!("This is a way to print a bunch of variables: {:?}", (x, y, z, status));

    let not_equals = 18 !=18;
    println!("The value os 18 != 18 is: {}", not_equals)
}
