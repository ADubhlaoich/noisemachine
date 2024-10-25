fn main() {

    let mut data = 42;           // data is a "mutable" variable, I.E, its value can change
    let ref1 = &mut data;   // ref1 is a variable which references the value of data
    let deref_copy = *ref1;      // deref_copy is a variable which makes a copy of ref1's values
    *ref1 = 13;                       // ref1 itself is mutable, and its value is updated
    println!("data is: {data}, deref_copy is {deref_copy}");
    // Since ref1 was referencing data the whole time, data is 13, while deref_copy is 42,
    // which is the value it copied at the earlier state

    let mut heap_data = vec![1, 2, 3];
    let ref1 = &mut heap_data;
    let deref_copy = ref1.clone();

    let move_out = ref1;
    let move_another_one = ref1;
}
