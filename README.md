# Rustifier: Python to Rust Code Conversion and Analysis
## Overview
The **Rustifier** is a tool designed to analyze Python code and convert it into Rust while ensuring compatibility with the rest of the codebase. It focuses on identifying challenges, risks, and optimizations involved in converting Python to Rust. The Rustifier agent communicates with the Groq model (via their API) to help generate Rust code and evaluate the feasibility of conversion. The system ensures a smooth transition from Python to Rust, offering insights and rewritten code where applicable.

## Features
* **Code Analysis:** The Rustifier tool evaluates Python code for compatibility with Rust, offering detailed feedback on conversion feasibility.
* **Python to Rust Conversion:** It generates Rust code equivalent to Python code while maintaining functionality and performance.
* **Compatibility Report:** For each Python file processed, the tool provides a report assessing the viability of conversion and any challenges faced.
* **System and User Prompts:** The tool uses advanced AI prompts to guide the process of analyzing Python files and generating Rust code.
* **Error Handling:** The tool provides logs and error messages for unsuccessful conversions.

## Prerequisites
Before using the Rustifier tool, ensure you have the following:
* **Python 3.8 or later** installed.
* **Groq API key** for accessing the Groq model. You must set this up in the environment variables.
* **Required Python libraries:**
  + `os`
  + `pathlib`
  + `dotenv`
  + `swarms`
  + `groq`

To install the necessary dependencies, you can run:
```bash
pip install swarms groq python-dotenv
```

### Installation
1. Clone this repository or download the code.
2. Set up your Python environment.
3. Set the required environment variables:
   * `GROQ_API_KEY`: Your Groq API key (this is necessary to use the Groq API for generating Rust code).
   ```bash
export GROQ_API_KEY="your-api-key-here"
```
4. Run the Rustifier tool on your Python codebase to convert Python files to Rust.

## How It Works
The Rustifier tool functions by reading Python code from a repository, sending each file to the Groq model to generate Rust equivalents, and saving the output as Rust files. It performs the following key steps:
### Directory Traversal
* The tool traverses the provided repository directory, identifying Python files (.py).
* It ensures each file is processed separately to avoid mixing code from different files.

### System and User Prompts
* The system prompt defines the high-level goal: analyze and convert Python code to Rust while ensuring compatibility with the repository.
* The user prompt provides the Groq model with the Python code and asks for a detailed analysis, including any potential challenges in converting the code and a Rust rewrite if possible.

### Rust Code Generation
* For each Python file, the tool communicates with the Groq model using the system and user prompts.
* The Groq model generates a Rust version of the Python code (if possible) and returns it.

### Saving and Organizing Outputs
* The Rustified code is saved in an output directory (`rustified_output`), preserving the structure of the original Python files.
* Each Python file is converted into a `.rs` file with the same name, but with a `.rustified` suffix added.

### Error Handling and Logging
* The tool logs any errors encountered during the conversion process.
* If a file cannot be converted, it will be skipped, and the error will be printed for debugging.

## Usage
To use the Rustifier, run the following script:
```bash
python rustifier.py
```
**Parameters:**
* `repo_path`: The directory containing Python files to be analyzed.
* `output_dir`: The directory where the Rustified code will be saved.

**Example:**
```bash
python rustifier.py
```
This will start the conversion process, processing the files in the `./swarms-master` directory and saving the output to the `./swarms-master-rustified` directory.

## Code Example
Here's a breakdown of the main components of the tool:
### Rustifier Agent and System Prompt
```python
system_prompt = (
    "You are an expert in analyzing and converting Python code into Rust. "
    "Your goal is to ensure compatibility and functionality without breaking interoperation "
    "with the rest of the repository. Provide detailed feedback on the compatibility of the code "
    "for conversion, including potential risks and limitations. "
    "For each Python file provided, attempt to rewrite it in Rust while maintaining its original behavior. "
    "At the top of the Rust file, include a comment indicating whether the conversion is viable, "
    "along with the reasoning behind the assessment. "
    "If only parts of the file can be converted, specify the portions and their Rust equivalents. "
    "Ensure compatibility with the rest of the project, highlight potential challenges, and provide "
    "in-line comments for clarity and maintainability."
)
```
This prompt is used to guide the Groq model in generating Rust code based on the provided Python files.

### File Traversal and Processing
```python
def traverse_repo(repo_path, output_dir):
    """Traverse the directory tree of the repo and process each file."""
    for root, _, files in os.walk(repo_path):
        for file in files:
            file_path = os.path.join(root, file)
            relative_path = os.path.relpath(file_path, repo_path)
            output_file_path = os.path.join(output_dir, relative_path)

            # Skip non-Python files
            if not file.endswith(".py"):
                continue

            print(f"Processing file: {file_path}")

            # Create output directory if it doesn't exist
            Path(output_file_path).parent.mkdir(parents=True, exist_ok=True)

            # Process the file
            try:
                process_file(file_path, output_file_path)
            except Exception as e:
                print(f"Error processing {file_path}: {e}")
```
This function walks through the repository directory and processes each Python file by sending it to the Groq API for analysis and conversion.

### Generating Rust Code
```python
def process_file(file_path, output_file_path):
    """Analyze and potentially translate the file to Rust."""
    with open(file_path, "r") as f:
        file_content = f.read()

    prompt = (
        f"Analyze the following Python file and determine if it can be converted to Rust "
        f"without breaking interoperation with the rest of the repository. Provide detailed "
        f"feedback on compatibility and rewrite it in Rust if possible. Highlight any "
        f"limitations or challenges, and include comments for clarity and maintainability.\n\n"
        f"{file_content}"
    )

    rustified_code = rustify_agent.llm(prompt)

    # Print the transformation to the console
    print("Transformation received from Groq:")
    print(rustified_code)

    # Save the Rustified code to the output file
    rust_output_file = output_file_path.replace(".py", "_rustified.rs")
    with open(rust_output_file, "w") as f:
        f.write(rustified_code)

    print(f"Saved Rustified file: {rust_output_file}")
```
This function processes each file, sends it to Groq, and saves the Rustified output to a new `.rs` file.

## Contribution
Contributions to improve the Rustifier tool are welcome! If you would like to help, please fork the repository, make your changes, and submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
