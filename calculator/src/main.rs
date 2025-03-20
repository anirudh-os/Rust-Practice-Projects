use std::io;

fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '%' | '^' => true,
        _ => false,
    }

    // "+-*/%^".contains(c) can also be used
}

fn extract_operands_and_operator(input: String) -> Result<(i32, char, i32), String> {
    let mut op1_str = String::new();
    let mut op2_str = String::new();
    let mut operator: Option<char> = None;

    let mut operator_found = false;

    for c in input.chars() {
        if is_operator(c) && operator_found {
            return Err("Two operators used!".to_string());
        }
        if is_operator(c) && !operator_found {
            operator = Some(c);
            operator_found = true; // Prevent assigning more than one operator
        } else if operator_found && c.is_numeric() {
            op2_str.push(c);
        } else if !operator_found && c.is_numeric() {
            op1_str.push(c);
        }
    }

    if let Some(op) = operator {
        let op1: i32 = op1_str.parse().map_err(|_| "Failed to parse the first operand")?;
        let op2: i32 = op2_str.parse().map_err(|_| "Failed to parse the second operand")?;
        Ok((op1, op, op2))
    } else {
        Err("No operator found".to_string())
    }
}

fn calculate (op1: i32, operator: char, op2:i32) -> Result<i32, String> {
    if op2 == 0 && operator == '/' {
        return Err("Divison by zero error!".to_string());
    }
    match operator {
        '+' => Ok(op1 + op2),
        '-' => Ok(op1 - op2),
        '*' => Ok(op1 * op2),
        '/' => Ok(op1 / op2),
        '%' => Ok(op1 % op2),
        '^' => Ok(op1.pow(op2 as u32)), // pow needs the second operand as a u32
        _ => Err("Unknown Operator!".to_string()),
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter the expression: ");

    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Error: {}!", e);
        return;
    }

    let result = extract_operands_and_operator(input);

    match result {
        Ok((op1, operator, op2)) => {
            let output = calculate(op1, operator, op2);
            match output {
                Ok(o) => println!("The result of the expression is {}", o),
                Err(e) => println!("An error occurred: {}!", e),
            }
        },
        Err(e) => println!("An error occurred: {}!", e),
    }
}
