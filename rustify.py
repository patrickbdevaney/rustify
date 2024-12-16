import os
WORKSPACE_DIR = "./"
os.environ["WORKSPACE_DIR"] = WORKSPACE_DIR
from pathlib import Path
from dotenv import load_dotenv
from swarms import Agent
from groq import Groq

# Load environment variables
load_dotenv()

# Initialize Groq client
api_key = os.getenv("GROQ_API_KEY")
if api_key is None:
    raise ValueError("GROQ_API_KEY environment variable is not set.")
client = Groq(api_key=api_key)

# Define the system prompt for the agent
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

# Define the agent for Rust compatibility analysis and transformation
rustify_agent = Agent(
    agent_name="RustifyAgent",
    llm=lambda prompt: client.chat.completions.create(
        messages=[
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": prompt}
        ],
        model="llama-3.3-70b-versatile"
    ).choices[0].message.content,
    max_loops=1,
    autosave=True,
    dashboard=False,
)

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

def main():
    # Define the paths
    repo_path = "./swarms-master"  # Input directory
    output_dir = "./swarms-master-rustified"  # Output directory

    if not os.path.exists(repo_path):
        raise FileNotFoundError(f"The repository path {repo_path} does not exist.")

        Path(output_dir).mkdir(parents=True, exist_ok=True)

    print("Starting the Rustification process...")
    traverse_repo(repo_path, output_dir)
    print("Rustification process completed.")

if __name__ == "__main__":
    main()
