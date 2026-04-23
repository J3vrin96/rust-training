mod maths;
mod restaurant;

fn main() {
    let result = maths::add(2, 5);
    restaurant::eat_at_restaurant();

    println!("{result}");
}
