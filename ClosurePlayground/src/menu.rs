use std::io;

pub fn display_menu_main() -> Result<i32, String> {
    println!("1. Arithmetic operations");
    println!("2. Dynamic Multiplier");
    println!("3. Custom Sorting");
    println!("4. Custom Filtering");
    println!("5. Memoized Fibonacci Calculator");
    println!("6. Stateful Closure");
    println!("7. Custom Map Function");
    println!("8. Event Handler Simulation");
    println!("9. Iterator Operations with Closures");
    println!("Enter your choice: ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse::<i32>() {
                Ok(choice) => Ok(choice),
                Err(e) => Err(format!("An error \"{}\" occurred!", e)),
            }
        },
        Err(e) => Err(format!("An error \"{}\" occurred!", e)),
    }
}

pub fn display_menu_sorting() {
    println!("1. Sort Ascending");
    println!("2. Sort Descending");
    println!("3. Return to main menu");
    println!("Enter your choice: ");
}

pub fn display_menu_filter() {
    println!("1. Even");
    println!("2. Odd");
    println!("3. Greater than a custom value");
    println!("4. Lesser than a custom value");
    println!("5. Divisible a custom value");
    println!("6. Return to main menu");
    println!("Enter your choice: ");
}