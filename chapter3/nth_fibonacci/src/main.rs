// Generate the nth Fibonacci number.

use std::io;

fn main() {
    println!("Infor the value of n");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    
    let n: u32 = n
        .trim()
        .parse()
        .expect("Please type a number");

    let mut fib_value = 0;
    let mut previous_value = fib_value;

    for i in 0..n {
        println!("{} : {}", i, fib_value);

        if i < 2 {
            fib_value = 1;
            previous_value = 1;
        } else {
            let temp = fib_value;

            fib_value = fib_value + previous_value;
            previous_value = temp;
        }
    }
}
