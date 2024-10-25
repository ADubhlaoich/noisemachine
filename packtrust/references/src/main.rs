fn main() {
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &mut heap_num;
    let ref2 = &mut heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    let mut heap_number = vec![1, 2, 3];
    let ref1 = &heap_num;
    let ref2 = &heap_num
    let ref3 = &mut heap_num;
    print!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref1, ref2, ref3);

    let mut heap_num = vec![7, 8, 9];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}.", ref1, ref2);
    let ref3 = &mut heap_num;
}