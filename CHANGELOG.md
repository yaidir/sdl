# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/).

---

## [1.0.0-alpha] - 2024-11-22

🚨 **Alpha Release Notice**:  
This is an **alpha release** of the SDL lexer. Features are still experimental and subject to change in future versions. Use with caution, and report issues to help us improve!

### Added

- **Lexer Implementation**:

  - Complete lexer for SDL programming language capable of tokenizing the following:
    - Keywords (`if`, `else`, `while`, `for`, etc.).
    - Identifiers and literals.
    - Operators and delimiters.
  - Support for single-line (`#`) and multi-line (`#[]`) comments.
  - Error handling for invalid tokens with detailed error messages.
  - Unit tests for tokenization with 95% code coverage.

- **Performance Optimization**:

  - Efficient tokenization using regex-based pattern matching.
  - Added caching for frequently used token patterns.

- **CLI Integration**:
  - Lexer can now be tested directly via the CLI (`sdl lexer <source-file>`).

### Changed

- Improved the error reporting format for better readability and consistency with future compiler stages.

### Fixed

- Resolved a bug where strings with escape sequences were incorrectly tokenized.
- Fixed edge cases where unclosed comments caused unexpected crashes.

---

## [Unreleased]

### Planned

- Syntax highlighting integration for lexer tokens.
- Additional token types for future language features.
- Lexer benchmarking suite for performance profiling.
