fn square(num: i32) -> i32 {
    return num * num; // OR num * num without semicolon at then end of the line (I prefer the return version :-D)
}

fn main() {
    let answer = square(3);

    println!("The square of 3 is {answer}");
}