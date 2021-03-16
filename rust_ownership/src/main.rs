fn borrow_variable() {
    // THIS IS NOT ALLOWED, BECAUSE THE VALUE WAS MOVED TO VARIABLE value2
    // let mut value = "Some".to_string();
    // value += " Value";
    // let valeu2 = value;
    // println!("{}", value);

    // this is allowed because int is considered simple values so they are copied into
    let value = 5;
    let value2 = value;
    println!("{}", value);
}

// Calculate string value without move the variable passed as parameter
fn string_size(string: &String) -> usize {
    string.len()
}

// Change string with mutable parameter
fn change(string: &mut String, value: &String) {
    string.push_str(value);
}

fn main() {
    let mut value = "Douglas".to_string();

    change( &mut value, &" Mendes".to_string());

    println!("{}", value);
}
