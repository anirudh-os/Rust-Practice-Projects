use std::collections::HashMap;
use std::io;

fn fib(n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    if n == 1 || n == 0 { return n }
    let prev_1: u32;
    let prev_2:u32;
    match cache.get(&(n - 1)) {
        Some(val) => {
            prev_1 = *val;
        },
        _ => {
            prev_1 = fib(n - 1, cache);
            cache.insert(n - 1, prev_1);
        }
    }
    match cache.get(&(n - 2)) {
        Some(val) => {
            prev_2 = *val;
        },
        _ => {
            prev_2 = fib(n - 2, cache);
            cache.insert(n - 2, prev_2);
        }
    }
    prev_1 + prev_2
}

pub fn memoization_fib() {
    println!("Enter the number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or(0);
    match input.trim().parse::<u32>() {
        Ok(number) => {
            if number == 0 {
                println!("0");
                return;
            }
            let mut hm = HashMap::new();
            println!("fib({}) = {}", number, fib(number, &mut hm));
        },
        Err(e) => {
            println!("An error \"{}\" occurred!", e);
            return;
        }
    }
}