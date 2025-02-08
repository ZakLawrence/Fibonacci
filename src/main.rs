use std::io;

fn fibonacci(n:u64)->u128{
    let mut numbers = (0,1);
    for _ in 1..n {
        let temp = numbers.1;
        numbers.1 = numbers.1+numbers.0;
        numbers.0 = temp;
    }
    numbers.1
}

fn main() {
    println!("Fibonacci Calulator");
    
    println!("What number do you want to calculate?");
    let mut input = String::new(); 
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input: u64 = input
                    .trim()
                    .parse()
                    .expect("Could not convert input");


    let fib = fibonacci(input);
    println!("The {input} fibonacci number is {fib}");
}
