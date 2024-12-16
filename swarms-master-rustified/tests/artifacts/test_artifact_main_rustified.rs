### Conversion Viability:
The provided Python code can be converted to Rust with some adjustments to accommodate Rust's ownership system, borrow checker, and typing system.

### Challenges and Limitations:
1. **Dynamic Typing**: Rust is a statically typed language, so the code needs to be typed explicitly.
2. **Lack of Direct Equivalent to Python's `datetime` Module**: Rust's `std::time` module provides basic time-related functionality. For more advanced datetime operations, you can use the `chrono` crate.
3. **Rust's `String` vs Python's `str`**: In Rust, `String` is a growable, owned string type, similar to Python's `str`.
4. **Error Handling**: Rust uses a strong focus on error handling and uses `Result` types to handle and propagate errors.
5. **Testing Framework**: The Python code uses `pytest`, which has no direct equivalent in Rust. However, the `#[cfg(test)]` attribute and the `assert` macro can be used for unit testing.

### Rust Equivalent:

```rust
// Import the necessary modules
use chrono::{DateTime, Utc};
use std::collections::Vec;

// Define the Artifact and FileVersion structs
#[derive(Debug)]
struct FileVersion {
    version_number: i32,
    content: String,
    timestamp: DateTime<Utc>,
}

#[derive(Debug)]
struct Artifact {
    file_path: String,
    file_type: String,
    contents: String,
    versions: Vec<FileVersion>,
    edit_count: i32,
}

impl Artifact {
    // Create a new artifact
    fn new(file_path: String, file_type: String) -> Self {
        Artifact {
            file_path,
            file_type,
            contents: String::new(),
            versions: Vec::new(),
            edit_count: 0,
        }
    }

    // Create a new version of the artifact
    fn create(&mut self, content: String) {
        let version = FileVersion {
            version_number: self.versions.len() as i32 + 1,
            content: content.clone(),
            timestamp: Utc::now(),
        };
        self.versions.push(version);
        self.contents = content;
    }

    // Edit the artifact
    fn edit(&mut self, content: String) {
        self.edit_count += 1;
        let version = FileVersion {
            version_number: self.versions.len() as i32 + 1,
            content: content.clone(),
            timestamp: Utc::now(),
        };
        self.versions.push(version);
        self.contents = content;
    }

    // Get a specific version of the artifact
    fn get_version(&self, version_number: i32) -> Option<&FileVersion> {
        self.versions.iter().find(|version| version.version_number == version_number)
    }

    // Get the contents of the artifact
    fn get_contents(&self) -> &str {
        &self.contents
    }

    // Get the version history of the artifact
    fn get_version_history(&self) -> String {
        let mut history = String::new();
        for version in &self.versions {
            history.push_str(&format!("Version {}\n", version.version_number));
        }
        history
    }

    // Convert the artifact to a dictionary-like representation
    fn to_dict(&self) -> serde_json::Value {
        serde_json::json!({
            "file_path": self.file_path,
            "file_type": self.file_type,
            "contents": self.contents,
            "versions": self.versions.iter().map(|version| serde_json::json!({
                "version_number": version.version_number,
                "content": version.content,
                "timestamp": version.timestamp,
            })).collect::<Vec<_>>(),
            "edit_count": self.edit_count,
        })
    }

    // Create an artifact from a dictionary-like representation
    fn from_dict(data: serde_json::Value) -> Self {
        let file_path = data["file_path"].as_str().unwrap().to_string();
        let file_type = data["file_type"].as_str().unwrap().to_string();
        let contents = data["contents"].as_str().unwrap().to_string();
        let mut versions = Vec::new();
        for version in data["versions"].as_array().unwrap() {
            versions.push(FileVersion {
                version_number: version["version_number"].as_i64().unwrap() as i32,
                content: version["content"].as_str().unwrap().to_string(),
                timestamp: DateTime::parse_from_str(version["timestamp"].as_str().unwrap(), "%+").unwrap(),
            });
        }
        let edit_count = data["edit_count"].as_i64().unwrap() as i32;
        Artifact {
            file_path,
            file_type,
            contents,
            versions,
            edit_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_version() {
        let version = FileVersion {
            version_number: 1,
            content: "Initial content".to_string(),
            timestamp: Utc::now(),
        };
        assert_eq!(version.version_number, 1);
        assert_eq!(version.content, "Initial content");
    }

    #[test]
    fn test_artifact_creation() {
        let artifact = Artifact::new("test.txt".to_string(), ".txt".to_string());
        assert_eq!(artifact.file_path, "test.txt");
        assert_eq!(artifact.file_type, ".txt");
        assert_eq!(artifact.contents, "");
        assert_eq!(artifact.versions.len(), 0);
        assert_eq!(artifact.edit_count, 0);
    }

    #[test]
    fn test_artifact_create() {
        let mut artifact = Artifact::new("test.txt".to_string(), ".txt".to_string());
        artifact.create("Initial content".to_string());
        assert_eq!(artifact.contents, "Initial content");
        assert_eq!(artifact.versions.len(), 1);
        assert_eq!(artifact.versions[0].content, "Initial content");
        assert_eq!(artifact.edit_count, 0);
    }

    #[test]
    fn test_artifact_edit() {
        let mut artifact = Artifact::new("test.txt".to_string(), ".txt".to_string());
        artifact.create("Initial content".to_string());
        artifact.edit("First edit".to_string());
        assert_eq!(artifact.contents, "First edit");
        assert_eq!(artifact.versions.len(), 2);
        assert_eq!(artifact.versions[1].content, "First edit");
        assert_eq!(artifact.edit_count, 1);
    }

    #[test]
    fn test_artifact_get_version() {
        let mut artifact = Artifact::new("test.txt".to_string(), ".txt".to_string());
        artifact.create("Initial content".to_string());
        artifact.edit("First edit".to_string());
        let version = artifact.get_version(1).unwrap();
        assert_eq!(version.content, "Initial content");
    }

    #[test]
    fn test_artifact_get_contents() {
        let mut artifact = Artifact::new("test.txt".to_string(), ".txt".to_string());
        artifact.create("Initial content".to_string());
        assert_eq!(artifact.get_contents(), "Initial content");
    }

    #[test]
    fn test_artifact_get_version_history() {
        let mut artifact = Artifact::new("test.txt".to_string(), ".txt".to_string());
        artifact.create("Initial content".to_string());
        artifact.edit("First edit".to_string());
        let history = artifact.get_version_history();
        assert!(history.contains("Version 1"));
        assert!(history.contains("Version 2"));
    }

    #[test]
    fn test_artifact_to_dict() {
        let mut artifact = Artifact::new("test.txt".to_string(), ".txt".to_string());
        artifact.create("Initial content".to_string());
        let dict = artifact.to_dict();
        assert_eq!(dict["file_path"].as_str().unwrap(), "test.txt");
        assert_eq!(dict["file_type"].as_str().unwrap(), ".txt");
        assert_eq!(dict["contents"].as_str().unwrap(), "Initial content");
        assert_eq!(dict["edit_count"].as_i64().unwrap(), 0);
    }

    #[test]
    fn test_artifact_from_dict() {
        let dict = serde_json::json!({
            "file_path": "test.txt",
            "file_type": ".txt",
            "contents": "Initial content",
            "versions": [
                {
                    "version_number": 1,
                    "content": "Initial content",
                    "timestamp": Utc::now(),
                }
            ],
            "edit_count": 0,
        });
        let artifact = Artifact::from_dict(dict);
        assert_eq!(artifact.file_path, "test.txt");
        assert_eq!(artifact.file_type, ".txt");
        assert_eq!(artifact.contents, "Initial content");
        assert_eq!(artifact.versions[0].content, "Initial content");
        assert_eq!(artifact.edit_count, 0);
    }
}
```

Note that this conversion assumes you have the `chrono` and `serde_json` crates installed. You can add them to your `Cargo.toml` file as follows:

```toml
[dependencies]
chrono = "0.4.19"
serde = { version = "1.0.118", features = ["derive"] }
serde_json = "1.0.64"
```