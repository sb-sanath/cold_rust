use std::io;

fn convert_fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    return ((fahrenheit - 32 as f32) * 5 as f32) / 9 as f32
}

fn convert_celcius_to_fahrenheit(celcius: f32) ->f32 {
    return (celcius * 1.8) + 32 as f32
}

fn main() {
    let mut fahrenheit = String::new();

    println!("Please enter the temperature in Farenheit");
    
    io::stdin().read_line(&mut fahrenheit).expect("Error reading input");
    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Error parsing number");

    let celcius: f32 = convert_fahrenheit_to_celcius(fahrenheit);
    println!("{} farenheit in celcius is : {}", fahrenheit, celcius);

    // 

    let mut celcius = String::new();

    println!("Please enter the temperature in Celcius");

    io::stdin().read_line(&mut celcius).expect("Error reading input");
    let celcius: f32 = celcius.trim().parse().expect("Error parsing number");

    let fahrenheit: f32 = convert_celcius_to_fahrenheit(celcius);
    println!("{} celcius in fahrenheit is : {}", celcius, fahrenheit);


}
