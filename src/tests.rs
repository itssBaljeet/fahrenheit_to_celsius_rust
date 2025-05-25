#[cfg(test)]
use super::*;

#[test]
fn test_boiling_point() {
    let ct = ConvertedTemp::fahrenheit_to_celsius(212.0);
    assert_eq!(ct.converted, 100.0);
}

#[test]
fn test_freezing_point() {
    let ct = ConvertedTemp::fahrenheit_to_celsius(32.0);
    assert_eq!(ct.converted, 0.0);
}