use std::io;

fn main() {
    println!("Enter Celcius temperature:");

    let mut celcius = String::new();

    io::stdin()
        .read_line(&mut celcius)
        .expect("Must be an string");

    let celcius: f32 = match celcius.trim().parse() {
        Ok(n) => n,
        Err(_) => 0.0,
    };

    let fahrenheit = temperatures_convertissor(celcius);

    println!("{celcius} celcius degrees equal {fahrenheit} fahrenheit degrees");

}

fn temperatures_convertissor(t: f32) -> f32 {
    return t * (9.0/5.0) + 32.0;
}
