# ccvault

A local, encrypted credit card vault for the command line. Card data is stored on disk encrypted with AES-256-GCM, using Argon2 for key derivation from a master password. Nothing leaves your machine.Cards are serialized to JSON, encrypted with AES-256-GCM, and stored at `~/.ccvault/vault.enc` by default. The encryption key is derived from your master password using Argon2, which is set upon first run. A random salt and nonce are generated for each save. The vault file is written with `0600` permissions (owner read/write only). Sensitive data in memory is zeroed on drop via the `zeroize` crate.

### Disclaimer 
CC-Vault is a personal project which started because I don't like to store my payment info in a browser. Storing your payment info locally reduces your attack surface in theory (although this depends entirely on your threat model) but for most people, browser-stored payment info is probably the more pragmatic choice given that it is usually built and audited by large security teams. But if you are one of those privacy nuts like me you might already be using tools like this (keepass for example).
If you like this idea and you are proficient in Rust and/or cryptography, please consider making a contribution via a PR or just general advice. Thank you!

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

options for field: `number`, `exp`, `cvv`, `name`, `zip`
## Building

Requires Rust. Clone and build:

```
git clone https://github.com/vlshields/cc-vault.git
cd cc-vault
cargo build --release
```

The binary will be at `target/release/cc-vault`. Copy it somewhere on your `$PATH`:

```
cp target/release/cc-vault ~/.local/bin/
```
or symlink
```
ln -s target/release/cc-vault ~/.local/bin/cc-vault
```
### Dependencies

```
sudo apt install xclip xdotool
```

### Running tests

```
cargo test
```


### To-Do
- Brute force protection
- xdotool autofill exposes card data in the process list
- auto-lock/session timeout


## License

This project is released under The Open Source Transparency License (OSTL)
