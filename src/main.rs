use temperature::{ConvertedTemp, Unit};

pub mod temperature;
pub mod util;
pub mod tests;

fn main() {
    println!("Temperature Converter");
    
    let unit: Unit = util::get_unit();

    let temperature: ConvertedTemp = util::get_temp(&unit);

    println!("{0} degrees in {1} is {2} degrees in {3}", temperature.original, unit, temperature.converted, unit.opposite())
}
