#![allow(unused_variables)]

use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = args[1].parse().unwrap();

    // assert_eq!(fib_rec(1), 1);
    // assert_eq!(fib_rec(2), 1);
    // assert_eq!(fib_rec(3), 2);
    // assert_eq!(fib_rec(4), 3);
    // assert_eq!(fib_rec(5), 5);
    // assert_eq!(fib_rec(6), 8);
    // assert_eq!(fib_rec(7), 13);
    // assert_eq!(fib_rec(8), 21);

    let mut time = Instant::now();
    println!(
        "Recursive: Fibonacci number {n} is: {}. In {}ms",
        fib_rec(n),
        time.elapsed().as_millis()
    );

    // assert_eq!(fib_iter(1), 1);
    // assert_eq!(fib_iter(2), 1);
    // assert_eq!(fib_iter(3), 2);
    // assert_eq!(fib_iter(4), 3);
    // assert_eq!(fib_iter(5), 5);
    // assert_eq!(fib_iter(6), 8);
    // assert_eq!(fib_iter(7), 13);
    // assert_eq!(fib_iter(8), 21);

    time = Instant::now();
    println!(
        "Iterative: Fibonacci number {n} is: {}. In {}ms",
        fib_iter(n),
        time.elapsed().as_millis()
    );
}

fn fib_rec(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    fib_rec(n - 1) + fib_rec(n - 2)
}

fn fib_iter(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }

    let mut prev2 = 0;
    let mut prev1 = 1;

    for i in 2..=n {
        let fib = prev1 + prev2;
        prev2 = prev1;
        prev1 = fib;
    }

    prev1
}
