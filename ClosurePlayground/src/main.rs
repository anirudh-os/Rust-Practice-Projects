mod arithmetic;
mod menu;
mod multiplier;
mod sorting;
mod filtering;
mod memoization;
mod stateful;

fn main() {
    match menu::display_menu_main() {
        Ok(choice) => {
            match choice {
                1 => arithmetic::arithmetic_operation(),
                2 => multiplier::dynamic_multiplier(),
                3 => sorting::sort_input(),
                4 => filtering::custom_filter(),
                5 => memoization::memoization_fib(),
                6 => stateful::stateful_closure(),
                _ => {}
            }
        },
        Err(e) => println!("{}",e),
    }
}
