# Contributing to ccvault

Thank you for considering a contribution to ccvault. This is a small, security-focused project and outside input is genuinely valuable, especially from people experienced with Rust and cryptography.

### What we're looking for

- Cryptography review and hardening (audit the encryption pipeline, key derivation, memory handling)
- Bug fixes, especially anything security-related
- Improvements to existing features (e.g. fixing the xdotool process list exposure, adding session timeout/auto-lock)
- Test coverage improvements
- Documentation improvements

### What we're NOT looking for

- Cloud sync, remote storage, or any feature that sends data off the machine. The entire point of ccvault is local-only storage.
- GUI or TUI wrappers. This is a CLI tool by design.


# Ground Rules

- Ensure changes compile and pass `cargo test` before submitting.
- Security-sensitive changes (encryption, key derivation, file permissions, memory handling) require extra scrutiny, expect thorough review.
- Don't add dependencies unless clearly justified. This project keeps its dependency tree small on purpose.
- Create an issue before starting work on major changes so the approach can be discussed first.
- Be respectful and constructive in all interactions.

# Your First Contribution

Not sure where to start? Look for issues labeled `good first issue` or `help wanted`. Some approachable areas:

Working on your first Pull Request? You can learn how from this free series, [How to Contribute to an Open Source Project on GitHub](https://egghead.io/series/how-to-contribute-to-an-open-source-project-on-github).

# Getting Started

1. Fork the repo and clone your fork.
2. Make sure you can build and run tests:
   ```
   cargo build
   cargo test
   ```
3. Create a branch for your changes.
4. Make your changes. Write tests if applicable.
5. Run `cargo test` and make sure everything passes.
6. Submit a pull request.

For small fixes (typos, formatting, comment cleanup), this process is all you need. For anything that changes behavior or touches crypto code, open an issue first to discuss the approach.

# How to Report a Bug

**If you find a security vulnerability, do NOT open an issue. Email vince@shieldsdev.biz directly instead.**



For non-security bugs, open a GitHub issue and include:

1. What OS and Rust version are you using?
2. What command did you run?
3. What did you expect to happen?
4. What actually happened?

# How to Suggest a Feature

ccvault's goal is simple: store credit card data locally, encrypted, with minimal attack surface. Feature suggestions should align with that.

Open an issue describing:
- The feature you'd like
- Why it's useful
- How it should work

Features that increase the attack surface or add network connectivity will almost certainly be declined.

# Code Review Process

Pull requests are reviewed by the maintainer. Expect feedback within a reasonable timeframe. After feedback is given, please respond within two weeks or the PR may be closed.

# Conventions

- Rust stable, formatted with `cargo fmt`
- Run `cargo clippy` before submitting
- Keep commits focused, one logical change per commit
- Write descriptive commit messages
