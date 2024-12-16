### Viability of Conversion
The conversion of the provided Python file to Rust is viable. 
The reasoning behind this assessment is that:
- Rust has built-in support for strings, dictionaries (equivalent to Rust's HashMap), lists (equivalent to Rust's Vec), tuples, and other data types.
- Rust's standard library provides functions for converting values to strings, such as `std::fmt::Display` and `std::fmt::Debug`.
- Recursion can also be implemented in Rust for handling nested data structures.

However, there are some potential challenges:
- Python's `str()` function can convert many types of objects to strings, but Rust requires more explicit type conversions.
- Rust's borrow checker and type system can make it more difficult to implement recursive functions that handle multiple types of data.

### Converted Rust Code

```rust
use std::collections::HashMap;
use std::fmt;

// Define a function to convert any data type to a string
pub fn any_to_str(data: &dyn fmt::Debug) -> String {
    match data {
        data as &str => format!("\"{}\"", data),
        data as &HashMap<_, _> => {
            let mut items = Vec::new();
            for (k, v) in data {
                let value = any_to_str(v);
                items.push(format!("{}: {}", k, value));
            }
            if items.is_empty() {
                "{}".to_string()
            } else {
                items.join(", ")
            }
        }
        data as &[_] => {
            let mut items = Vec::new();
            for v in data {
                items.push(any_to_str(v));
            }
            if items.is_empty() {
                "[]".to_string()
            } else {
                items.join(", ")
            }
        }
        data as &(_) => {
            format!("{:?}", data)
        }
        _ => "".to_string(),
    }
}

// Define a function to convert any data type to a string with recursion
pub fn any_to_str_recursive(data: &dyn fmt::Debug) -> String {
    match data {
        data as &str => format!("\"{}\"", data),
        data as &HashMap<_, _> => {
            let mut items = Vec::new();
            for (k, v) in data {
                let value = any_to_str_recursive(v);
                items.push(format!("{}: {}", k, value));
            }
            if items.is_empty() {
                "{}".to_string()
            } else {
                items.join("\n")
            }
        }
        data as &[_] => {
            let mut items = Vec::new();
            for v in data {
                items.push(any_to_str_recursive(v));
            }
            if items.is_empty() {
                "[]".to_string()
            } else {
                format!("[{}]", items.join(", "))
            }
        }
        data as &(_) => {
            format!("{:?}", data)
        }
        _ => "".to_string(),
    }
}

// Define a function to convert None to an empty string
pub fn none_to_str(data: &Option<&dyn fmt::Debug>) -> String {
    match data {
        Some(_) => "".to_string(),
        None => "".to_string(),
    }
}

// Example usage:
fn main() {
    let example_dict = HashMap::from([
        ("name", "John"),
        ("age", 30),
        ("hobbies", vec!["reading", "hiking"]),
    ]);

    let nested_dict = HashMap::from([
        (
            "user",
            HashMap::from([
                ("id", 123),
                (
                    "details",
                    HashMap::from([
                        ("city", "New York"),
                        ("active", true),
                    ]),
                ),
            ]),
        ),
        ("data", vec![1, 2, 3]),
    ]);

    println!("Dictionary:");
    println!("{}", any_to_str_recursive(&example_dict));

    println!("\nNested Dictionary:");
    println!("{}", any_to_str_recursive(&nested_dict));

    println!("\nList and Tuple:");
    println!("{}", any_to_str_recursive(&vec![1, "text", None, (1, 2)]));
    println!("{}", any_to_str_recursive(&(true, false, None)));

    println!("\nEmpty Collections:");
    println!("{}", any_to_str_recursive(&vec![]));
    println!("{}", any_to_str_recursive(&HashMap::new()));
}
```
Note: The provided Rust code is a conversion of the given Python code. However, due to Rust's type system and borrow checker, it's a bit less flexible and may need more explicit type conversions.