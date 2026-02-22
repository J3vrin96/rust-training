fn main() {
    let mut s1 = String::from("hello");
    let len: usize = calculate_length(&s1);

    update_value(&mut s1); // should fails because s1 is not mutable

    
    println!("{len}");
    println!("{s1}");

    print_again(&s1);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn update_value(s: &mut String) {
    return s.push_str(" my friend");
}

fn print_again(s: &String) {
    println!("{s}")
}