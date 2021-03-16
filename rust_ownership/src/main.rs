fn borrow_variable() {
    // THIS IS NOT ALLOWED, BECAUSE THE VALUE WAS MOVED TO VARIABLE value2
    // let mut value = "Some".to_string();
    // value += " Value";
    // let valeu2 = value;
    // println!("{}", value);

    // this is allowed because int is considered simple values so they are copied into
    let value = 5;
    let _ = value;
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

fn change_with_string_mutation() {
    let mut value = "Douglas".to_string();
    change( &mut value, &" Mendes".to_string());
    println!("{}", value);
}

fn mutiple_mutable_pointers_to_same_address() {
    let mut string = String::from("Hello buddy");

    // Not allowed
    //let string_ref2 = &mut string;
    //let string_ref = &mut string;

    // Not allowed
    //let string_ref2 = &string;
    //let string_ref = &mut string;

    let string_ref = &string;
    let string_ref2 = &string;
    println!("{}, {}", string_ref, string_ref2);

    // This is allowed because these scopes don’t overlap, so this code is allowed.
    let string_ref3 = &mut string;

    println!("{}", string_ref3);
}


fn main() {
    change_with_string_mutation();
}
