# Contributing

Thank you for your interest in contributing to `tauri-plugin-thermal-printer`.

Quick guide:

- **Report an issue**: Open an issue describing the expected behavior, the observed behavior, and minimal reproduction steps. Include OS and versions of Tauri/Cargo/Node when relevant.
- **Propose changes (PR)**:
  - Fork the repository and create a branch with a clear name: `feature/<description>`, `fix/<description>`, or `chore/<description>`.
  - Keep commits small and use descriptive commit messages.
  - Run `cargo build --release` to verify Rust builds.
  - If you changed JS/TS artifacts, run `bun install` and `bun run build` from the repository root.
  - Add tests where applicable and describe the changes in your PR.
- **Formatting and style**:
  - Rust: run `cargo fmt` and avoid breaking the public API.
  - JS/TS: follow the project's `tsconfig`, ESLint, and formatting rules.
- **PR review**: Maintainers may request changes before merging. Please respond to comments and update your branch.

Additional notes:

- Check the `examples/` folder for usage patterns and manual test cases.
- If you modify native integrations (Android/iOS), provide platform-specific build instructions in the PR description.

Thank you for helping improve the project.

