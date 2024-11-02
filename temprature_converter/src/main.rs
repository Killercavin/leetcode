use std::io;

fn main() {
    println!("Enter the temperature in Fahrenheit or Celsius:");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    }

    // Trim the input to remove trailing newline character
    let input = input.trim();

    // Parse the input string to a floating-point number
    let temp: f64 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please enter a valid number.");
            return;
        }
    };

    let fahrenheit_temp: f64 = (temp - 32.0) * 5.0 / 9.0;
    let celsius_temp: f64 = (temp * 9.0 / 5.0) + 32.0;

    // Print the results
    println!("{} Fahrenheit is equal to {:.2} Celsius", temp, fahrenheit_temp);
    println!("{} Celsius is equal to {:.2} Fahrenheit", temp, celsius_temp);
}