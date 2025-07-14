# Global Information Tracker (Rust)

A Rust-based Git-compatible implementation of the Global Information Tracker, a simplified version of the Git version control system built from scratch. This project aims to help understand the internal mechanics of Git by reimplementing its core functionalities in Rust.

## Features

*   Object storage for blobs, trees, and commits
*   Basic command support: init, hash-object, cat-file, write-tree, commit-tree, etc.
*   SHA-1 hashing for object identification
*   Filesystem structure mimicking .git internals
*   Tree serialization/deserialization
*   Simple command-line interface

## Commands Implemented

| Command | Description |
| :--- | :--- |
| `git init` | Initializes a new Git repository |
| `git hash-object` | Stores file content as a blob object |
| `git cat-file` | Outputs the contents of a Git object |
| `git write-tree` | Creates a tree object from the current index |
| `git commit-tree` | Creates a commit object from a tree |

## Repository Structure

```
.
├── src/
│   ├── cli.rs             # CLI argument parsing and command handlers
│   ├── commands/          # Individual command implementations
│   │   ├── cat_file.rs
│   │   ├── commit_tree.rs
│   │   ├── hash_object.rs
│   │   ├── init.rs
│   │   └── write_tree.rs
│   ├── git.rs             # Core Git object model and logic
│   └── main.rs            # Entry point
├── .gitignore
├── Cargo.toml
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License.
