# Contributing to dbtective 🕵️

Thank you for your interest in contributing to dbtective! We're excited to have you join our detective squad.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)
- [just](https://github.com/casey/just) (optional, for convenient task running)

### Setting up your development environment

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:

   ```bash
   git clone https://github.com/your-username/dbtective.git
   cd dbtective
   ```

3. **Set up the upstream remote**:

   ```bash
   git remote add upstream https://github.com/Felix-Blom/dbtective.git
   ```

4. **Install dependencies and build**:

   ```bash
   cargo build
   ```

5. **Run the application**:

   ```bash
   # Debug mode (includes debug logs and timing)
   cargo run

   # Release mode (optimized, clean output)
   cargo run --release

   # With arguments
   cargo run -- --help
   cargo run -- run --verbose
   ```

### Using just (optional)

If you have [just](https://github.com/casey/just) installed, you can use these convenient commands:

```bash
# Run in debug mode
just run

# Run in release mode
just run-release

# Run with arguments
just run --verbose
just run-release run --help
```

## Development Workflow

### 1. Create an Issue

Before starting work, please:

- Check if an issue already exists for your idea
- Create a new issue describing the feature, bug, or improvement
- Wait for discussion and approval before starting significant work

### 2. Branch Strategy

- Create a feature branch from `main`:

  ```bash
  git checkout main
  git pull upstream main
  git checkout -b feature/your-feature-name
  ```

- Use descriptive branch names:
  - `feature/add-yaml-parsing`
  - `fix/logging-timestamp-format`
  - `docs/update-installation-guide`

### 3. Development Guidelines

#### Code Style

- Follow Rust conventions and use `cargo fmt` to format code
- Run `cargo clippy` to catch common mistakes
- Write clear, self-documenting code with meaningful variable names
- Add comments for complex logic

#### Testing

- Write unit tests for new functionality
- Ensure all tests pass: `cargo test`
- Add integration tests where appropriate

#### Logging

- Use appropriate log levels:
  - `debug!()` for verbose/debug messages
  - `info!()` for general information
  - `warn!()` for warnings
  - `error!()` for errors

#### Error Handling

- Use `Result<T, E>` for fallible operations
- Provide meaningful error messages
- Use `anyhow` for error handling where appropriate

### 4. Commit Guidelines

We use [gitmoji](https://gitmoji.dev/) or [howmoji](https://github.com/Felix-Blom/howmoji) for expressive commit messages. This makes the commit history more readable and fun!

**Examples:**

```
✨ feat: add YAML configuration parsing

- Implement ConfigParser for dbtective.yml
- Add validation for rule definitions
- Include error handling for malformed configs

Closes #123
```

```
🐛 fix: resolve logging timestamp format issue

- Fix timestamp format in verbose mode
- Ensure consistent time display across log levels

Fixes #456
```

**Example gitmoji/howmoji:**

- ✨ `:sparkles:` for new features
- 🐛 `:bug:` for bug fixes
- 📝 `:memo:` for documentation changes

**Tools to help:**

- [Gitmoji CLI](https://github.com/carloscuesta/gitmoji-cli): `npm i -g gitmoji-cli`
- [Howmoji](https://github.com/Felix-Blom/howmoji): Felix's own gitmoji tool!

**Traditional format (also acceptable):**

- `feat:` for new features
- `fix:` for bug fixes
- `docs:` for documentation changes
- `style:` for formatting changes
- `refactor:` for code refactoring
- `test:` for adding tests
- `chore:` for maintenance tasks

### 5. Pull Request Process

1. **Update your branch** with the latest main:

   ```bash
   git checkout main
   git pull upstream main
   git checkout your-feature-branch
   git rebase main
   ```

2. **Run the full test suite**:

   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

3. **Create a Pull Request** with:

   - Clear title describing the change
   - Detailed description of what was changed and why
   - Reference to related issues (e.g., "Closes #123")
   - Screenshots for UI changes (if applicable)

4. **Respond to feedback** promptly and make requested changes

## Project Structure

```
dbtective/
├── src/
│   ├── main.rs           # Application entry point
│   ├── cli/              # Command-line interface
│   │   ├── mod.rs
│   │   └── commands.rs   # CLI command definitions
│   └── core/             # Core functionality
│       ├── mod.rs
│       ├── logging.rs    # Logging configuration
│       ├── parse_config.rs
│       └── parse_pyproject.rs
├── tests/                # Integration tests
├── Cargo.toml           # Rust package configuration
├── justfile             # Task runner commands
└── README.md
```

## Areas for Contribution

### High Priority

- [ ] dbt manifest parsing and analysis
- [ ] Rule engine implementation
- [ ] Configuration file parsing (YAML)
- [ ] Output formatters (JSON, markdown, HTML)

### Medium Priority

- [ ] Performance optimizations
- [ ] Additional linting rules
- [ ] Documentation improvements
- [ ] CI/CD pipeline enhancements

### Good First Issues

- [ ] Improve error messages
- [ ] Add more unit tests
- [ ] Documentation updates
- [ ] Code cleanup and refactoring

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Getting Help

- **Questions?** Open a discussion on GitHub
- **Bugs?** Create an issue with a minimal reproduction case
- **Ideas?** Start with an issue to discuss the approach

## Recognition

Contributors will be recognized in:

- The project's README
- Release notes for significant contributions
- The project's website (coming soon!)

---

**Happy detecting!** 🕵️‍♀️🔍

Thank you for helping make dbtective better for everyone!
