# ccvault

A local, encrypted credit card vault for the command line. Card data is serialized to JSON, encrypted with AES-256-GCM, and stored at `~/.ccvault/vault.enc`. The encryption key is derived from your master password using Argon2. Each save uses a fresh random salt and nonce, writes the vault atomically, stores the vault file with `0600` permissions, and keeps `~/.ccvault` at `0700`.

Sensitive data in memory is zeroed where practical via the `zeroize` crate. Card number and CVV prompts are hidden. Autofill avoids passing card data as command-line arguments to `xdotool`.

### Disclaimer

This personal project does not claim to be more secure than browser storage.

## Usage

```
ccvault add                          # Add a card interactively
ccvault list                         # List saved card labels
ccvault show <label>                 # Show full card details
ccvault remove <label>               # Remove a card
ccvault copy <label> <field>         # Copy a field to clipboard (cleared after 10s)
ccvault fill <label>                 # Autofill card fields via xdotool
ccvault change-password              # Re-encrypt vault with a new password
```

Options for `<field>`: `number`, `exp`, `cvv`, `name`, `zip`.

After repeated failed password attempts, ccvault locks further attempts by writing `~/.ccvault/lockout.json`. It does not delete your vault file.

## Building

Requires Rust. Clone and build:

```
git clone https://github.com/vlshields/cc-vault.git
cd cc-vault
cargo build --release
```

The binary will be at `target/release/ccvault`. Copy it somewhere on your `$PATH`:

```
cp target/release/ccvault ~/.local/bin/
```

or symlink:

```
ln -s target/release/ccvault ~/.local/bin/ccvault
```

### Dependencies

`xclip` is required for `ccvault copy`; `xdotool` is required for `ccvault fill`.

```
sudo apt install xclip xdotool
```

### Running checks

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo audit
```

### To-Do

- auto-lock/session timeout

## License

MIT
