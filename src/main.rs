use std::io;

fn main() {
    println!("Hello, world!");
    println!("And welcome to my first toy CLI program in Rust!");

    let (old_unit, new_unit): (String, String) = get_unit();

    let (old_temp, new_temp): (i32, i32) = get_temp(&new_unit);

    println!("{old_temp} degrees in {old_unit} is {new_temp} degrees in {new_unit}")
}

fn fahrenheit_to_celsius(temp: i32) -> i32 {
    (temp - 32) * 5/9
}

fn celsius_to_fahrenheit(temp: i32) -> i32 {
    (temp * 9/5) + 32
}

fn get_unit() -> (String, String) {
    println!("Convert to Celsius or Fahrenheit? (c/f)");

    loop {
        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line!");

        match unit.trim().to_lowercase().as_str() {
        "c" => {
            break (String::from("Fahrenheit"), String::from("Celsius"))
            },
        "f" => {
            break (String::from("Celsius"), String::from("Fahrenheit"))
            },
        _ => {
            println!("Please input 'c' or 'f' only!");
            continue;
            },
        };
    }
}

fn get_temp(unit: &str) -> (i32, i32) {
    println!("What temperature do you want to convert from?");

    loop {
        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line!");

        let temp: i32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("NaN");
                continue;
            }
        };

        match unit {
            "Fahrenheit" => return (temp, celsius_to_fahrenheit(temp)),
            "Celsius" => return (temp, fahrenheit_to_celsius(temp)),
            _ => panic!("How the hell did you get here?"),
        }
    }
}