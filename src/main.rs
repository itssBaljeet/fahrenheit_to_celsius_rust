use std::io;

use temperature::{ConvertedTemp, Unit};

pub mod temperature;
pub mod tests;

fn main() {
    println!("Hello, world!");
    println!("And welcome to my first toy CLI program in Rust!");

    let unit: Unit = get_unit();

    let temperature: ConvertedTemp = get_temp(&unit);

    println!("{0} degrees in {1} is {2} degrees in {3}", temperature.original, unit, temperature.converted, unit.opposite())
}

fn get_unit() -> Unit {
    println!("Convert to Celsius or Fahrenheit? (c/f)");

    loop {
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line!");

        match unit.trim().to_lowercase().as_str() {
        "c" => {
            return Unit::Celsius
            },
        "f" => {
            return Unit::Fahrenheit
            },
        _ => {
            println!("Please input 'c' or 'f' only!");
            continue
            },
        };
    }
}

fn get_temp(unit: &Unit) -> ConvertedTemp {
    println!("Enter temperature in {}", unit.opposite());

    loop {
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line!");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue
            }
        };

        match unit {
            Unit::Fahrenheit => return ConvertedTemp::celsius_to_fahrenheit(temp),
            Unit::Celsius => return ConvertedTemp::fahrenheit_to_celsius(temp),
        }
    }
}