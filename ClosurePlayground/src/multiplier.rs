use std::io;

fn return_closure(multiplier: i32) -> impl Fn(i32) -> i32 {
    move |num| num * multiplier
}


pub fn dynamic_multiplier() {
    println!("===============================");
    println!("       Dynamic Multiplier      ");
    println!("-------------------------------");
    println!("Create a custom multiplier once,");
    println!("then reuse it as many times as you like!");
    println!("===============================");

    println!("Enter the multiplier: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let multiplier = input.trim().parse::<i32>().unwrap_or(0);
    if multiplier == 0 {
        println!("Incorrect multiplier!");
        return;
    }

    loop {
        println!("1. Multiply");
        println!("2. Return to main menu");
        println!("Enter your choice: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>().unwrap_or(0) {
            1 => {
                let product = return_closure(multiplier);
                println!("Enter the number: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let number = input.trim().parse::<i32>().unwrap_or(0);
                if number == 0 {
                    println!("Incorrect number!");
                    continue;
                }
                println!("The product of {} and {} is {}", multiplier, number, product(number));
            },
            2 => {
                println!("Returning to the main menu...");
                return;
            },
            _ => println!("Wrong option chosen!!")
        }
    }
}