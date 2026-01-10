use std::io;

fn fib(n: u32) {

}

fn main() {

    let n: u32 = loop {

        let mut input = String::new();
        
        println!("Enter the nth number");

        io::stdin().read_line(&mut input).expect("Error reading input");

        let input: u32 = match input.trim().parse() {
                        Ok(num) => break num,
                        Err(_) => {
                            println!("Please enter a whole number");
                            continue;
                        }
        };
    };

    fib(n);
}
