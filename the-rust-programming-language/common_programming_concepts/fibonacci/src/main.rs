fn main() {
    for i in 0..15 {
        let result = fibonacci(i);

        println!("Fibonacci result is: {result}");
    };
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}