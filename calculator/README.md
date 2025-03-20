# CLI Expression Evaluator in Rust

A simple command-line expression evaluator built in Rust.  
It parses input expressions like `12+4`, `18*3`, or `2^5`, extracts operands and operator, and performs the calculation.

---

## Features

- Supports basic arithmetic operations: `+`, `-`, `*`, `/`, `%`, `^`
- Parses input like `25+5` or `100/4`
- The expressions can have spaces too!
- Graceful error handling for invalid input formats
- Demonstrates Rust concepts like input parsing, pattern matching, error propagation, and `Result` handling

---

## Usage

### Build and Run

```bash
cargo build
cargo run
```

### Example

```
Enter the expression:
12*8
The result of the expression is 96
```

---

## Concepts Demonstrated

- Pattern matching
- String parsing and input validation
- Safe numeric operations with error handling
- CLI interaction using `std::io`

---

## Project Structure

```text
src/
├── main.rs       # Core logic for input parsing and evaluation
Cargo.toml        # Project metadata and dependencies
```

---

## Future Improvements

- Support for floating-point operands (e.g., `3.14*2`)
- Chained expressions with parentheses (e.g., `2*(3+4)`)
- More operator support: bitwise, logical, etc.
- Replace `i32` with `BigInt` for large exponentiations
- Unit tests for parser and calculation logic

---

## License

This project is licensed under the MIT License.

