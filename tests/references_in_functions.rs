// OWNERSHIP
// move semantics

fn print_count(country_name: &String) {
    println!("My country is {}", country_name);
}
pub fn references_in_functions() {
    let country = "대한민국".to_string();
    print_count(&country);
    print_count(&country);
    print_count(&country);
}

