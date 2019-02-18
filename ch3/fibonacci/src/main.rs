fn main() {
    let n = 100;
    println!("n'th fibonacci number (n={}): {}", n, fib(n));
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
