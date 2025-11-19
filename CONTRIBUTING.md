# Contributing to telemetry-kit

Thank you for your interest in contributing to telemetry-kit! 🎉

This project is in **early development**, and we welcome contributions of all kinds.

## 🚀 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork**: `git clone https://github.com/YOUR_USERNAME/telemetry-kit.git`
3. **Create a branch**: `git checkout -b feature/your-feature-name`
4. **Make your changes**
5. **Run tests**: `cargo test`
6. **Commit your changes**: `git commit -am 'Add some feature'`
7. **Push to your fork**: `git push origin feature/your-feature-name`
8. **Create a Pull Request**

## 📋 What We're Looking For

### High Priority

- **Core API Design**: Help shape the public API
- **Privacy Features**: Anonymization, sanitization, GDPR compliance
- **Documentation**: Examples, tutorials, API docs
- **CLI Tools**: Scaffolding, auto-instrumentation suggestions
- **Testing**: Unit tests, integration tests, examples

### Medium Priority

- **Backend Integrations**: OTLP, custom exporters
- **Performance**: Benchmarks, optimizations
- **Examples**: Real-world use cases
- **CI/CD**: GitHub Actions, automated releases

### Future

- **Self-hosted Server**: Collection server implementation
- **Dashboards**: Web UI for analytics
- **VS Code Extension**: Inline instrumentation suggestions

## 🎯 Project Goals

Keep these in mind when contributing:

1. **Privacy First**: Never compromise user privacy
2. **Simple API**: Keep it as simple as possible
3. **Batteries Included**: Provide sensible defaults
4. **CLI Friendly**: Optimize for command-line tools
5. **Self-Hostable**: Users should be able to run their own infrastructure

## 📐 Code Style

- Follow standard Rust formatting: `cargo fmt`
- Run clippy before committing: `cargo clippy`
- Add tests for new functionality
- Document public APIs with doc comments
- Keep dependencies minimal

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy -- -D warnings
```

## 📝 Documentation

- Add doc comments to public items
- Update README.md if adding user-facing features
- Add examples to `examples/` directory
- Update CHANGELOG.md

## 🐛 Reporting Bugs

When reporting bugs, please include:

1. **Description**: Clear description of the issue
2. **Reproduction**: Minimal code to reproduce
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Environment**: OS, Rust version, telemetry-kit version

## 💡 Suggesting Features

We love feature suggestions! Please:

1. Check existing issues first
2. Describe the use case
3. Explain how it fits the project goals
4. Consider privacy implications
5. Suggest an API design if possible

## 🤝 Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

## 📜 License

By contributing, you agree that your contributions will be licensed under the [MIT License](./LICENSE)

## 🙏 Thank You

Every contribution matters, whether it's:
- Reporting a bug
- Suggesting a feature
- Writing documentation
- Submitting code
- Spreading the word

Thank you for making telemetry-kit better! 🚀
