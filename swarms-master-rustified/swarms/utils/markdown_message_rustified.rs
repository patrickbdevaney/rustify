```rust
// Viable: Yes, but requires additional dependencies for formatted output and string manipulation.
// Reasoning: The provided Python code can be converted to Rust, but Rust does not have a direct equivalent to Python's built-in string formatting and color output. 
// We will use the `ansi_term` crate for colored output and `lazy_static` with `regex` for any potential string formatting.

use ansi_term::{Color, Style};
use std::io;

// Define a Formatter struct to mimic the behavior of the Python formatter
struct Formatter {
    // Initialize the formatter with default colors
}

impl Formatter {
    fn new() -> Self {
        Formatter {}
    }

    // Implement a method to print a panel with the given line
    fn print_panel(&self, line: &str) {
        println!("{}", line);
    }

    // Implement a method to print a panel with a line of dashes
    fn print_dashes(&self, count: usize) {
        println!("{}", "-".repeat(count));
    }
}

// Function to display markdown message
fn display_markdown_message(message: &str, color: &str) {
    // Create a new formatter instance
    let formatter = Formatter::new();

    // Define colors based on the input color string
    let color_style = match color.to_lowercase().as_str() {
        "cyan" => Style::new(Color::Cyan),
        "red" => Style::new(Color::Red),
        "green" => Style::new(Color::Green),
        "yellow" => Style::new(Color::Yellow),
        _ => Style::default(), // Default to the standard console color
    };

    // Iterate over each line in the message
    for line in message.lines() {
        line = line.trim();
        if line.is_empty() {
            // If the line is empty, print a newline
            println!();
        } else if line == "---" {
            // If the line is a series of dashes, print a panel of dashes
            formatter.print_dashes(50);
        } else {
            // For other lines, print the line itself
            println!("{}", color_style.paint(line));
        }
    }

    // Check if the message is a single line starting with '>'
    if !message.contains('\n') && message.starts_with('>') {
        // If so, print an additional newline for aesthetic purposes
        println!();
    }
}

fn main() {
    // Example usage
    display_markdown_message("Hello, World!", "cyan");
    display_markdown_message("> This is a markdown message", "green");
    display_markdown_message("---\nThis is a multiline message\nwith multiple lines", "red");
}
```

**Notes on conversion:**

* The original Python code uses a `formatter` object from `swarms.utils.formatter`, which is not a standard Python module. We've created a simple `Formatter` struct in Rust to mimic its behavior.
* Python's `print()` function automatically appends a newline, whereas Rust's `println!()` macro does the same. However, Rust's `print!()` macro does not append a newline, so we use `println!()` throughout this code.
* For colored output, we use the `ansi_term` crate in Rust, which provides a similar functionality to Python's color output libraries.
* The code has been modified to use Rust's `match` statement for color selection, rather than using Python's conditional statements.
* We've added a `main` function to demonstrate example usage of the `display_markdown_message` function.
* Error handling has not been explicitly implemented in this code. In a real-world application, you would want to handle potential errors, such as invalid color inputs.

**Challenges and limitations:**

* This conversion assumes that the input message string is not overly large, as Rust may incur performance penalties for large string operations.
* The `ansi_term` crate used for colored output may not work correctly in all environments, such as certain IDEs or text editors.
* The `Formatter` struct created in Rust does not have the same level of customization as the original Python `formatter` object, and may need additional modifications to match the original behavior.
* As with any conversion between languages, there may be subtle differences in behavior or edge cases that are not immediately apparent. Thorough testing is recommended to ensure the Rust code behaves as expected.