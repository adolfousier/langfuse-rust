# Contributing to langfuse-rust

Thank you for your interest in contributing to the langfuse-rust project! This document provides guidelines and instructions to help you get started.

## Code of Conduct

- Be respectful and considerate in all interactions
- Provide constructive feedback
- Focus on the issue, not the person
- Follow Rust community best practices

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- Git
- A GitHub account

### Setting Up Your Development Environment

1. Fork the repository on GitHub:
   - Visit https://github.com/adolfousier/langfuse-rust
   - Click the "Fork" button in the top-right corner

2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_GB_USERNAME/langfuse-rust.git
   cd langfuse-rust
   ```

3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/adolfousier/langfuse-rust.git
   ```

4. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Making Changes

1. Make your changes in your feature branch
2. Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
3. Add or update tests as needed
4. Update documentation to reflect your changes

### Running Tests

Run the test suite:
```bash
cargo test
```

### Formatting Code

Format your code using rustfmt:
```bash
cargo fmt
```

### Checking Your Code

Run Clippy to catch common mistakes:
```bash
cargo clippy
```

## Submitting Changes

### Creating a Pull Request

1. Push your changes to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Go to the original repository on GitHub
3. Click "Pull Request" and then "New Pull Request"
4. Select your fork and the feature branch
5. Fill out the PR template with details about your changes

### Pull Request Guidelines

- Keep PRs focused on a single topic
- Include tests for new functionality
- Update documentation as needed
- Link to any related issues

## Release Process

The maintainers will handle versioning and releases according to:

- PATCH version for backwards-compatible bug fixes
- MINOR version for new functionality in a backwards-compatible manner
- MAJOR version for incompatible API changes

## Getting Help

If you need help at any point:

- Open an issue on GitHub with the "question" label
- Ask in the pull request comments
- Reach out to the maintainers

## Project Structure

```
langfuse_tracker/
├── src/
│   ├── lib.rs           # Library entry point and exports
│   ├── client.rs        # API client implementation
│   ├── error.rs         # Error types and handling
│   ├── types.rs         # Core data structures
│   └── utils/           # Utility functions
│       └── mod.rs
├── examples/            # Example code
│   └── simple.rs
├── tests/               # Integration tests
│   └── integration.rs
├── README.md
└── CONTRIBUTING.md      # This file
```

## License

By contributing to this project, you agree that your contributions will be licensed under the project's MIT license.

Thank you for contributing to langfuse_tracker!