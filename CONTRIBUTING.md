# Contributing to dbtective 🕵️

Thank you for your interest in contributing to dbtective! We're excited to have you join our detective squad.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)
- [just](https://github.com/casey/just) (optional, all relevant commands are shown in the `justfile`).
- For running documentation locally:
   - [golang](https://go.dev/doc/install)
   - [hugo](https://gohugo.io/installation/)

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

Use the commands shown in the `justfile` or install [just](https://github.com/casey/just) and run:

   ```bash
   just run
   just run-verbose
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

- Write unit tests in the corresponding file.
- Write integration tests in the test folder.
- Ensure all tests pass: `cargo test`
- Add integration tests where appropriate


### 4. Commit Guidelines

Please use [commitizen](https://commitizen-tools.github.io/commitizen/) or use the same style to write informative commit messages using traditional format.


### 5. Pull Request Process

1. **Update your branch** with the latest main:

   ```bash
   git rebase -i  origin/main
   ```

2. **Run the full test suite**:

Use the commands shown in the `justfile` or install [just](https://github.com/casey/just) and run:
   ```bash
   just test
   just lint
   just fmt
   ```

3. **Create a Pull Request** 

4. **Code Review**

5. **Merge!**

## Documentation

To update documentation please refer to the `/docs/content` folder and apply markdown changes.
Use the commands shown in the `justfile` or install [just](https://github.com/casey/just) and run:

```bash
just docs
```


## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Getting Help

- **Questions?** Open a discussion on GitHub
- **Bugs?** Create an issue with a minimal reproduction case
- **Ideas?** Start with an issue to discuss the approach

## Recognition

Contributors will be recognized in:
- Release notes for significant contributions

**Happy detecting!** 🕵️‍♀️🔍

Thank you for helping make dbtective better for everyone!
