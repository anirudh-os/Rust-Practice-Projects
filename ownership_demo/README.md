# Ownership, Borrowing, and Mutability in Rust

This project demonstrates three key concepts in Rust:

- **Ownership Transfer**: Passing a `String` by value moves ownership.
- **Borrowing**: Passing a reference allows read-only access without moving ownership.
- **Mutable Borrowing**: Passing a mutable reference allows modifying the original value.

The example includes three functions:

- `transfer_ownership(string: String)` – Takes ownership and prints it.
- `borrow_ownership(string: &String)` – Borrows the string immutably.
- `mutate_string(string: &mut String)` – Mutably borrows and modifies the string.

This is useful for understanding how Rust ensures memory safety without a garbage collector.
