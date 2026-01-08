use std::io;

fn convert_fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    ((fahrenheit - 32.0) * 5.0) / 9.0
}

fn convert_celcius_to_fahrenheit(celcius: f32) ->f32 {
    (celcius * 1.8) + 32.0
}

fn read_user_input() -> f32 {
    let mut user_input  = String::new();

    io::stdin().read_line(&mut user_input).expect("Error reading input");

    user_input.trim().parse().expect("Error parsing number")

}

fn main() {

    println!("Please enter the temperature in Farenheit");

    let fahrenheit: f32 = read_user_input();
    let celcius: f32 = convert_fahrenheit_to_celcius(fahrenheit);

    println!("{} farenheit in celcius is : {}", fahrenheit, celcius);

    // 

    println!("Please enter the temperature in Celcius");

    let celcius: f32 = read_user_input();
    let fahrenheit: f32 = convert_celcius_to_fahrenheit(celcius);
    println!("{} celcius in fahrenheit is : {}", celcius, fahrenheit);


}
