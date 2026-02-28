# Contributing to ccvault

Thank you for considering a contribution to ccvault. This is a small, security-focused project and outside input is genuinely valuable, especially from people experienced with Rust and cryptography.

### Help Needed!

- Cryptography review and hardening (audit the encryption pipeline, key derivation, memory handling)
- Bug fixes, especially anything security-related
- Improvements to existing features (e.g. fixing the xdotool process list exposure, adding session timeout/auto-lock)
- Test coverage improvements
- Documentation improvements




# Ground Rules

- Ensure changes compile and pass `cargo test` before submitting.
- Security-sensitive changes (encryption, key derivation, file permissions, memory handling) require extra scrutiny, expect thorough review.
- Don't add dependencies unless clearly justified. This project keeps its dependency tree small on purpose.
- Create an issue before starting work on major changes so the approach can be discussed first.
- Be respectful and constructive in all interactions.
- **Beginners are welcome!**
  

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

# Conventions

- Rust stable, formatted with `cargo fmt`
- Run `cargo clippy` before submitting
- Keep commits focused, one logical change per commit
- Write descriptive commit messages
