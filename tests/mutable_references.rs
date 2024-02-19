pub fn mutable_references() {
    // error
    // let mut number = 10;
    // let number_ref = &number;
    // let number_change = &mut number;
    // *number_change += 10;
    // println!("{}", number_ref);

    // success
    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    let number_ref = &number;
    println!("{}", number_ref);
}