fn main() {
    let number = 5;
    let inline_if_statement = if number == 6 { 1 } else { 0 };

    if number < 5 {
        println!("Number is less than five");
    } else if number == 5 {
        println!("Number is equal to five");
    } else {
        println!("Number is bigger than five");
    }

    println!("{inline_if_statement}");

    // infinite_loop();

    conditional_loop();
    while_loop_array();
}
// Infinite loop
fn infinite_loop() {
    loop {
        println!("again!");
    }
}

// Stop when condition is true
fn conditional_loop() {
    let mut count = 0;

    loop {
        println!("Current count value: {count}");
        count += 1;

        if count == 5 {
            println!("Stop");
            break;
        }
    }
}

fn while_loop_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
