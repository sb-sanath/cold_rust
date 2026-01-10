use std::io;

fn fib(n: u32) -> u32 {
    if n < 1 {
        return n;
    }

    let mut a: u32 = 0;
    let mut b: u32 = 1;

    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

fn main() {

    let n: u32 = loop {

        let mut input = String::new();
        
        println!("Enter the nth number");

        io::stdin().read_line(&mut input).expect("Error reading input");

        match input.trim().parse() {
                        Ok(num) => break num,
                        Err(_) => {
                            println!("Please enter a whole number");
                            continue;
                        }
        };
    };

    let n = fib(n);
    println!("The fionacci sum is {}", n);
}
