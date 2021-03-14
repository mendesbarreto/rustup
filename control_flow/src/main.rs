
fn if_let_method() {
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("{}", number);
}


fn return_value_from_loop() {
    let mut counter = 0;
    let max_interaction = 10;

    let result = loop {
        counter += 1;

        if counter == max_interaction {
            break counter
        }
    };

    println!("{}", result);
}

fn main() {
    return_value_from_loop();
}
