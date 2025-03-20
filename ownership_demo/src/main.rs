fn transfer_ownership(string: String) {
    println!("Printing from the transfer function: {}", string);
}

fn borrow_ownership(string: &String) {
    println!("Printing from the borrow function: {}", string);
}

fn mutate_string(string: &mut String) {
    string.push_str(" World!");
    println!("Printing from the mutate function: {}", string);
}

fn main() {
    // Ownership transfer
    let str1 = String::from("Hello");
    transfer_ownership(str1);
    // println!("{}", str1); // ❌ This would cause a compile error (ownership moved)

    // Borrowing
    let str2 = String::from("Hello");
    borrow_ownership(&str2);
    println!("After borrowing, still accessible in main: {}\n", str2);

    // Mutable borrowing
    let mut str3 = String::from("Hello");
    mutate_string(&mut str3);
    println!("After mutation, value in main: {}", str3);
}
