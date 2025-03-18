use std::io;
use crate::Unit::{Celsius, Fahrenheit};

/// Enum representing temperature units.
#[derive(Debug)]
enum Unit {
    Celsius,
    Fahrenheit,
}

impl Unit {

    /// Converts a string to a `Unit` (Celsius or Fahrenheit).
    ///
    /// This function trims any leading or trailing whitespace from the string,
    /// converts it to lowercase, and matches it to either "Celsius" or "Fahrenheit".
    /// If a valid match is found, it returns the corresponding `Unit`. Otherwise,
    /// it returns an error with the message "Not a valid unit!".
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice representing the unit name ("Celsius" or "Fahrenheit").
    ///
    /// # Returns
    ///
    /// * `Ok(Unit)` - The corresponding temperature unit (`Unit::Celsius` or `Unit::Fahrenheit`).
    /// * `Err(&'static str)` - If the string does not match "Celsius" or "Fahrenheit".

    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.trim().to_lowercase().as_str() {
            "celsius" => Ok(Celsius),
            "fahrenheit" => Ok(Fahrenheit),
            _ => Err("Not a valid unit!"),
        }
    }

    /// Takes a string containing both a numerical value and a unit, and returns a tuple with the parsed `Unit` and the value.
    ///
    /// The function splits the string into the numeric value and the unit part. The numeric part is expected to be an integer,
    /// and the unit part should be either "Celsius" or "Fahrenheit". It returns a tuple `(Unit, i32)` if successful, or an error
    /// message if something goes wrong.
    ///
    /// # Arguments
    ///
    /// * `s` - A string containing both a number and a unit, e.g., "25Celsius" or "100Fahrenheit".
    ///
    /// # Returns
    ///
    /// * `Ok((Unit, i32))` - The parsed unit and value.
    /// * `Err(String)` - An error message if parsing fails, either due to an invalid value or unit.

    fn value_and_unit(s: String) -> Result<(Unit, i32), String> {
        let mut unit_str = String::new();
        let mut value_str = String::new();

        // Split the string into value and unit
        for c in s.chars() {
            if c.is_digit(10) {
                value_str.push(c);
            } else {
                unit_str.push(c);
            }
        }

        // Parse the value part
        let value = value_str
            .parse::<i32>()
            .map_err(|e| format!("The value must be an integer! Error: {}", e))?;

        // Parse the unit part
        let unit = Unit::from_str(&unit_str)
            .map_err(|e| format!("Error: {}", e))?;

        Ok((unit, value))
    }

    // Convert the values
    fn convert(unit: Unit, value: i32) -> (Unit, i32) {
        match unit {
            Celsius => (Fahrenheit, (value * 9) / 5 + 32),
            Fahrenheit => (Celsius, (value - 32) * 5 / 9),
        }
    }
}

/// Main function that handles user input, processes it, and displays the result.
fn main() {
    let mut input = String::new();

    println!("Enter the temperature you want to convert (value unit): ");

    // Reading user input. The `?` operator is used here to automatically handle any errors.
    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Error reading line: {}", e);
        return;
    }

    // Process the input string, calling `Unit::value_and_unit`.
    let result_temp = Unit::value_and_unit(input);

    match result_temp {
        Ok((unit, value)) => {
            let (converted_unit, converted_value) = Unit::convert(unit, value);
            println!("{} {:?} ",  converted_value, converted_unit);
        },
        Err(err) => println!("Error: {}", err),
    }
}