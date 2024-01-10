pub fn references_and_shadowing() {
    // shadowing
    let country = "대한민국";
    let country_ref = &country;

    println!("{:p}", &country_ref);
    println!("{:p}", &country);

    // println!("{country_ref}, {country}");
}