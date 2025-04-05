use std::io;
use crate::menu::display_menu_sorting;

pub fn extract_numbers(input: &str) -> Vec<i32> {
    let output: Vec<i32> = input.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
    output
}
pub fn sort_input() {
    loop {
        display_menu_sorting();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<i32>().unwrap_or(0);
        match choice {
            1 => {
                println!("Enter the list of numbers you want to sort in a line, separated by space: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut list = extract_numbers(input.trim());
                list.sort_by(|a, b| a.cmp(b));
                println!("The sorted list is {:?}", list);
            },
            2 => {
                println!("Enter the list of numbers you want to sort in a line, separated by space: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut list = extract_numbers(input.trim());
                list.sort_by(|a, b| b.cmp(a));
                println!("The sorted list is {:?}", list);
            },
            3 => return,
            _ => {
                println!("Wrong choice!");
            }
        }
    }
}