use std::io;

fn main() {
    println!("Fahrenheit to Celsius Converter");

    // Gets the coice to convert from
    println!("Convert From Fahrenheit or from celsius? \n1 - Fahrenheit \n2 - Celsius");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
   
    // Parse the input string into an integer
    let number: i32 = choice.trim().parse().expect("Invalid input");

    if number == 1{
    
    // Read input from the user
    println!("Enter the temperature in Fahrenheit: ");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    // Convert the input to a floating-point number
    let fahrenheit: f32 = fahrenheit.trim().parse().expect("Invalid input");

    // Convert Fahrenheit to Celsius
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    // Display the result
    println!("Equivalent temperature in Celsius: {:.2}°", celsius);

} else if number == 2 {

    // Read input from the user
    println!("Enter the temperature in Celsius: ");
    let mut celsius = String::new();
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    // Convert the input to a floating-point number
    let celsius: f32 = celsius.trim().parse().expect("Invalid input");

    // Convert Celsius to Fahrenheit
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

    // Display the result
    println!("Equivalent temperature in Fahrenheit: {:.2}°", fahrenheit);
    }
}
