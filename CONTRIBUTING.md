# Contributing to Hexx

Contributions are welcomed ! On this repository you may:

- Report a bug by opening an Github issue
- Submit a fix by opening a Pull request
- Propose new features either in an issue or in a pull request

## Pull requests

Pull requests are the best way to propose changes to the codebase.

1. Fork the repo and create your branch from `main`.
2. If you've added code, please add unit tests.
3. If you've changed APIs, please update the documentation. If you changed the
top level documentation, sync the `README.md` with [cargo-sync-readme](https://crates.io/crates/cargo-sync-readme).
4. Ensure the test suite passes.
5. Make sure `clippy`, `rustdoc` and `rustmft` are happy.
6. Add an entry in the `CHANGELOG.md` under the `Unreleased` section describing
your changes
7. Open that pull request with a clear description of the work done

### Github Actions

When you open a pull requests, various workflows will check your contribution:

- The `Rust` workflow will check that the following checks pass, both for the
source code and examples
  - `cargo clippy`
  - `cargo build`
  - `cargo fmt`
  - `cargo rustdoc`
  - `cargo test`
- The `Docs` workflow will check:
  - That you added a `CHANGELOG.md` entry
  - That both `README.md` and `src/lib.rs` global documentation are in sync.

## Any contributions you make will be under the Apache 2.0 Software License

In short, when you submit code changes, your submissions are understood to be
under the same [License](./LICENSE) that covers the project.
