// Convert temperatures between Fahrenheit and Celsius.
use std::io;
fn main() {
    println!("Temperature Converter");
    println!("1: Convert Fahrenheit to Celsius");
    println!("2: Convert Celsius to Fahrenheit");
    let mut choice = String::new();
    println!("Enter your choice (1 or 2): ");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid input. Please enter 1 or 2");
            return ;
        }
    };
    if choice == 1{
        convert_fahrenheit_to_celsius();
    }  else if choice == 2 {
        convert_celsius_to_fahrenheit();
    } else {
        println!("Invalid choice. Please enter 1 or 2.");
    }
}

fn convert_fahrenheit_to_celsius() {
    let mut fahrenheit = String::new();
    println!("Enter temperature in Fahrenheit: ");
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read input");

    let fahrenheit: f64 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F is equal to {celsius:.2}°C");
}

fn convert_celsius_to_fahrenheit() {
    let mut celsius = String::new();
    println!("Enter temperature in Celsius: ");
    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read input");

    let celsius: f64 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{celsius}°C is equal to {fahrenheit:.2}°F");
}


