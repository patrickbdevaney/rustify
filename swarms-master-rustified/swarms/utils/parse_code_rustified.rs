### Conversion Viability: 
The provided Python file can be converted to Rust with minimal modifications, taking into account the differences in string handling, regular expressions, and dictionaries between the two languages. However, because Rust's regex engine does not support the same level of complexity as Python's, we'll need to ensure our pattern is correctly interpreted.

### Rust Conversion: 

```rust
// viability: Viable with modifications for regex and dictionary handling.
// Potential risks include differences in regex pattern matching and potential issues with Unicode handling,
// though these can be mitigated with the right crates and considerations.

use regex::Regex;
use std::collections::HashMap;

/// Extracts all code blocks from Markdown text along with their languages.
///
/// Args:
///     markdown_text (str): The input Markdown text.
///
/// Returns:
///     Vec<HashMap<String, String>>: A vector of HashMaps, each containing:
///         - 'language': The detected language (or 'plaintext' if none specified).
///         - 'content': The content of the code block.
pub fn extract_code_blocks_with_language(markdown_text: &str) -> Vec<HashMap<String, String>> {
    let pattern = Regex::new(r"```(\w+)?\n(.*?)```").unwrap();
    let captures = pattern.captures_iter(markdown_text);
    
    // Parse results
    let mut code_blocks: Vec<HashMap<String, String>> = Vec::new();
    for capture in captures {
        let language: String = capture.get(1).map_or("plaintext".to_string(), |m| m.as_str().trim().to_string());
        let content = capture.get(2).unwrap().as_str().trim().to_string();
        let mut block = HashMap::new();
        block.insert("language".to_string(), language);
        block.insert("content".to_string(), content);
        code_blocks.push(block);
    }

    code_blocks
}

/// Extracts content of code blocks for a specific language or all blocks if no language specified.
///
/// Args:
///     markdown_text (str): The input Markdown text.
///     language (str, optional): The language to filter by (e.g., 'yaml', 'python').
///
/// Returns:
///     str: The concatenated content of matched code blocks or an empty string if none found.
pub fn extract_code_from_markdown(markdown_text: &str, language: Option<&str>) -> String {
    // Get all code blocks with detected languages
    let code_blocks = extract_code_blocks_with_language(markdown_text);

    // Filter by language if specified
    let filtered_blocks: Vec<String> = match language {
        Some(lang) => code_blocks
            .into_iter()
            .filter(|block| block.get("language").unwrap() == lang)
            .map(|block| block.get("content").unwrap().clone())
            .collect(),
        None => code_blocks
            .into_iter()
            .map(|block| block.get("content").unwrap().clone())
            .collect(),
    };

    // Return concatenated content
    if filtered_blocks.is_empty() {
        String::new()
    } else {
        filtered_blocks.join("\n\n")
    }
}
```

### Limitations and Challenges:
- **Regex Engine:** While Rust's regex engine (`regex` crate) is powerful, it might interpret patterns slightly differently than Python's `re` module, requiring adjustments in complex patterns.
- **String Handling:** Rust enforces ownership and borrowing, which can lead to more verbose code compared to Python when handling strings and their manipulation.
- **Dictionary/HashMap:** Rust's `HashMap` is the equivalent of Python dictionaries, but accessing and manipulating its elements requires more explicit handling (e.g., `get`, `insert`) and error handling for cases like `None`.
- **Error Handling:** Rust requires explicit handling of potential errors (like pattern compilation failure in `Regex::new`), which can add complexity compared to Python's approach.

### Usage Example:
```rust
fn main() {
    let markdown_text = "```python
print('Hello, world!')
```

```yaml
key: value
```
";
    println!("{:?}", extract_code_blocks_with_language(markdown_text));
    println!("{}", extract_code_from_markdown(markdown_text, Some("python")));
    println!("{}", extract_code_from_markdown(markdown_text, None));
}
```
This Rust code maintains the original behavior of the Python functions. Note that to run this code, you need to add `regex` as a dependency in your `Cargo.toml`:
```toml
[dependencies]
regex = "1"
```