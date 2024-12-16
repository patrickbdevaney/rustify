```rust
// Viable for conversion: Yes
// Reasoning: The Python code utilizes standard libraries and can be translated
//             to Rust, but some Python-specific features may require additional work.
//             The `subprocess` and `logging` functionality have direct Rust equivalents,
//             but the `black` library for code formatting is written in Python, so a similar
//             Rust library would be needed.

use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use log::{info, error, warn};
use log::LevelFilter;
use env_logger;

use rustfmt::{Config, Edition};

// Rust has a built-in logger, but it's not as feature-rich as loguru.
// We'll use the `log` crate and `env_logger` for a similar experience.

// Simplified token counting using a simple Rust wrapper
struct Tokenizer;

impl Tokenizer {
    fn count_tokens(&self, output: &str) -> usize {
        // Simple token count: each token is separated by a space.
        output.split_whitespace().count()
    }
}

pub struct CodeExecutor {
    max_output_length: usize,
    artifacts_dir: String,
    language: String,
    tokenizer: Tokenizer,
}

impl CodeExecutor {
    pub fn new(
        max_output_length: usize,
        artifacts_directory: String,
        language: String,
    ) -> Self {
        // Create the artifacts directory if it doesn't exist.
        fs::create_dir_all(&artifacts_directory).expect("Failed to create directory");

        // Initialize the logger with the required level.
        env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .init();

        // Initialize the executor with the given parameters.
        Self {
            max_output_length,
            artifacts_dir: artifacts_directory,
            language,
            tokenizer: Tokenizer,
        }
    }

    fn format_code(&self, code: &str) -> Result<String, String> {
        // Rustfmt is the Rust equivalent of the Python black library.
        let mut config = Config::default();
        let edition = Edition::Edition2018;
        let formatted_code = rustfmt::format_input(config, edition, code).unwrap();
        Ok(formatted_code)
    }

    pub fn execute(&self, code: &str) -> Result<String, String> {
        // Format the code before execution.
        let formatted_code = match self.format_code(code) {
            Ok(code) => code,
            Err(e) => {
                error!("Error formatting code: {}", e);
                return Err(format!("Error formatting code: {}", e));
            }
        };

        info!("Executing code:\n{}", formatted_code);

        // Execute the code and capture the output.
        let output = match Command::new(&self.language)
            .arg("-c")
            .arg(formatted_code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
        {
            Ok(output) => output,
            Err(e) => {
                error!("Error executing code: {}", e);
                return Err(format!("Error executing code: {}", e));
            }
        };

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).unwrap();
            error!("Error executing code: {}", stderr);
            return Err(format!("Error executing code: {}", stderr));
        }

        let output = String::from_utf8(output.stdout).unwrap();
        info!("Code output:\n{}", output);

        // Count the tokens in the output.
        let token_count = self.tokenizer.count_tokens(&output);
        println!("Token count: {}", token_count);

        if self.max_output_length > 0 && token_count > self.max_output_length {
            warn!("Output length exceeds {} characters. Truncating output.", self.max_output_length);
            return Ok(format!("{}...", &output[..self.max_output_length - 3]));
        }

        Ok(output)
    }
}

fn main() -> Result<(), String> {
    let executor = CodeExecutor::new(300, "artifacts".to_string(), "python3".to_string());

    let code = r#"
import requests
from typing import Any

def fetch_financial_news(api_key: str, query: str, num_articles: int) -> Any:
    try:
        url = f"https://newsapi.org/v2/everything?q={query}&apiKey={api_key}"
        response = requests.get(url)
        response.raise_for_status()
        return response.json()
    except requests.RequestException as e:
        print(f"Request Error: {e}")
        raise
    except ValueError as e:
        print(f"Value Error: {e}")
        raise

api_key = ""
result = fetch_financial_news(api_key, query="Nvidia news", num_articles=5)
print(result)
"#;

    match executor.execute(code) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
```

This Rust equivalent maintains the same functionality and behavior as the provided Python code. However, there are a few limitations and challenges to consider:

1.  **Code Formatting:** The original Python code uses the `black` library for code formatting, which is written in Python and doesn't have a direct Rust equivalent. In this example, the `rustfmt` library is used instead, which provides similar functionality but might not work exactly the same way as `black`.
2.  **Token Counting:** The provided Python code uses a `TikTokenizer` to count tokens in the output. In the Rust version, a simplified token counting function is implemented, which splits the output string by whitespace and counts the resulting substrings. This might not be as accurate as the `TikTokenizer` but provides a basic token counting functionality.
3.  **Subprocess Execution:** The Rust code uses the `Command` API to execute the Python code, which is similar to the `subprocess` module in Python.
4.  **Error Handling:** Error handling is implemented using `Result` types in Rust, which provides a more explicit and safe way of handling errors compared to the Python version.