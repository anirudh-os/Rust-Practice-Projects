# Temperature Converter CLI (Rust)

A simple command-line temperature converter built with Rust.\
Takes user input like `100 Celsius` or `212 Fahrenheit`, parses the value and unit, and converts it to the other scale.

---

## Features

- Convert between Celsius and Fahrenheit
- Handles input like `25 Celsius` or `77 Fahrenheit`
- Graceful error handling for invalid input
- Demonstrates Rust concepts like enums, pattern matching, and input parsing

---

## Usage

### Build and Run

```bash
cargo build
cargo run
```

### Example

```
Enter the temperature you want to convert (value unit):
100 Fahrenheit
37 Celsius
```

---

## Concepts Demonstrated

- Enums and pattern matching
- Custom parsing and input validation
- Basic error handling using `Result` and `map_err`
- CLI interaction using `std::io`

---

## Directory Structure

```text
src/
├── main.rs       # All logic in a single binary crate
Cargo.toml        # Project metadata and dependencies
```

---

## Future Ideas

- Add support for floating-point values (e.g., `36.6Celsius`)
- Loop for multiple conversions
- Optional CLI args using `clap` or `argh`

---
