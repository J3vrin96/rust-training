fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_parameter(15); // argument = 15
    another_function_with_parameters(5, 'i');
    
    let add_one = plus_one(5);
    let add_two = plus_two(add_one);

    println!("add_ione equal: {add_one} and add_two equal: {add_two}");
}

fn another_function() {
    println!("Hello from another function!");
}

// parameter = x
fn another_function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn another_function_with_parameters(x: i32, letter: char) {
    println!("Letter {letter} appear in the number {x} ?");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}

fn plus_two(x: i32) -> i32 {
    return x + 2;
}
