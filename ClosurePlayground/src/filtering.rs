use std::io;
use crate::sorting;
use crate::menu;
use sorting::extract_numbers;
use menu::display_menu_filter;

fn filter_and_print<F>(list: &[i32], condition: F)
where F: Fn(i32) -> bool,
{
    let filtered_list: Vec<i32> = list.iter().copied().filter(|x| condition(*x)).collect();
    println!("The values which satisfy the condition are {:?}", filtered_list);
}


pub fn custom_filter() {
    println!("Enter the list of numbers you want to sort in a line, separated by space: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let list = extract_numbers(input.trim());
    loop {
        display_menu_filter();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<i32>().unwrap_or(0);
        match choice {
            1 => {
                let condition = |x:i32| x % 2 == 0;
                filter_and_print(&list, condition);
            },
            2 => {
                let condition = |x:i32| x % 2 != 0;
                filter_and_print(&list, condition);
            },
            3 => {
                println!("Enter the custom value to compare: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                    Ok(number) => {
                        let condition = |x: i32| x > number;
                        filter_and_print(&list, condition);
                    },
                    Err(e) => println!("An error \"{}\" occurred!", e),
                }
            },
            4 => {
                println!("Enter the custom value to compare: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                    Ok(number) => {
                        let condition = |x: i32| x < number;
                        filter_and_print(&list, condition);
                    },
                    Err(e) => println!("An error \"{}\" occurred!", e),
                }
            },
            5 => {
                println!("Enter the custom value to compare: ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<i32>() {
                    Ok(number) => {
                        let condition = |x: i32| x % number == 0;
                        filter_and_print(&list, condition);
                    },
                    Err(e) => println!("An error \"{}\" occurred!", e),
                }
            },
            6 => {
                println!("Returning to the main menu...");
                return;
            }
            _ => {}
        }
    }
}