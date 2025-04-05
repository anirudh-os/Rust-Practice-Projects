use std::io;
use crate::menu::display_menu_stateful;

fn return_closure() -> impl FnMut() -> i32 {
    let mut counter = 0;
    move || {
        counter += 1;
        counter
    }
}

pub fn stateful_closure() {
    let mut stateful = return_closure();
    loop {
        display_menu_stateful();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<i32>() {
            Ok(1) => {
                let counter = stateful();
                println!("The number of times closure has been called is: {}", counter);
            },
            Ok(2) => {
                println!("Returning to the main menu..");
                return;
            },
            _ => {
                println!("Wrong choice!");
            }
        }
    }
}