use core::fmt;

#[derive(Debug)]
pub enum Unit {
    Celsius,
    Fahrenheit,
}

impl Unit {
    pub fn opposite(&self) -> Self {
        match self {
            Unit::Celsius => Unit::Fahrenheit,
            Unit::Fahrenheit => Unit::Celsius,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Unit::Celsius => "Celsius",
            Unit::Fahrenheit => "Fahrenheit",
        })
    }
}

#[derive(Debug)]
pub struct ConvertedTemp {
    pub original: f64,
    pub converted: f64,
}

impl ConvertedTemp {
    pub fn fahrenheit_to_celsius(temp: f64) -> Self {
        Self {
            original: temp,
            converted: (temp - 32.0) * 5.0/9.0,
        }
    }

    pub fn celsius_to_fahrenheit(temp: f64) -> Self {
        Self {
            original: temp,
            converted: (temp * 9.0/5.0) + 32.0,
        }
        
    }
}
