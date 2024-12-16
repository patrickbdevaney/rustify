### Viability of Conversion
The provided Python file can be converted to Rust. The main challenge lies in finding Rust equivalents for the Python libraries and data structures used, such as `datetime`, `pydantic`, and Python's built-in `Dict`. 

Rust's standard library provides a `std::collections::HashMap` that can be used to represent dictionaries. For handling dates and times, we can use the `chrono` crate, which provides a robust implementation of date and time functionality.

The `pydantic` library, which is used for building robust, scalable data models, does not have a direct Rust equivalent. However, Rust's ownership system and borrow checker ensure memory safety and can be used to create robust data models. We can use Rust's built-in data validation features, such as pattern matching and the `Result` type, to achieve similar functionality.

### Rust Equivalent
```rust
// Viable for conversion: Yes, using chrono crate for datetime and std::collections::HashMap for metadata dictionary.
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Define a Message struct to represent a message with timestamp and optional metadata.
#[derive(Debug)]
struct Message {
    /// Timestamp when the message was created
    timestamp: DateTime<Utc>,
    /// Sender of the message
    sender: String,
    /// Content of the message
    content: String,
    /// Optional metadata associated with the message
    metadata: Option<HashMap<String, String>>,
}

impl Message {
    // Create a new Message instance
    fn new(sender: String, content: String, metadata: Option<HashMap<String, String>>) -> Self {
        Self {
            timestamp: Utc::now(),
            sender,
            content,
            metadata,
        }
    }

    // Return a string representation of the message
    fn to_string(&self) -> String {
        match &self.metadata {
            Some(metadata) => {
                // If metadata is present, include it in the string representation
                format!("{} - {}: {} (metadata: {:?})", self.timestamp, self.sender, self.content, metadata)
            }
            None => {
                // If metadata is not present, exclude it from the string representation
                format!("{} - {}: {}", self.timestamp, self.sender, self.content)
            }
        }
    }
}

fn main() {
    // Create a new message
    let message = Message::new("Kye".to_string(), "message".to_string(), None);
    
    // Print the message
    println!("{}", message.to_string());
}
```
### Limitations and Challenges

* The `pydantic` library's validation features and automatic generation of JSON serialization code are not directly equivalent in Rust. You would need to manually implement data validation and serialization using Rust's built-in features.
* Rust's `std::collections::HashMap` is not exactly equivalent to Python's `Dict`. While both are hash-based data structures, Rust's `HashMap` provides additional functionality like lazy loading of values.
* Rust's borrow checker and ownership system may require additional effort to ensure memory safety when working with complex data structures.
* Error handling in Rust is different from Python. Rust uses the `Result` type and the `?` operator for error propagation, while Python uses try-except blocks.

Overall, the conversion of the provided Python code to Rust is viable with careful consideration of the differences in language features and libraries.