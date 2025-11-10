# Shell Setup for Development

## Cargo Environment

After installing Rust, the Cargo environment needs to be loaded in your shell.

### Automatic Loading (Recommended)

Add to your shell config file:

**For bash (~/.bashrc):**
```bash
source "$HOME/.cargo/env"
```

**For zsh (~/.zshrc):**
```bash
source "$HOME/.cargo/env"
```

**For fish (~/.config/fish/config.fish):**
```fish
source "$HOME/.cargo/env.fish"
```

Then reload your shell:
```bash
source ~/.bashrc  # or ~/.zshrc
```

### Manual Loading (Current Session Only)

If you don't want to modify your shell config, load it manually each session:

```bash
source "$HOME/.cargo/env"
```

### Verification

Check if Cargo is available:

```bash
cargo --version  # Should show: cargo 1.91.1
rustc --version  # Should show: rustc 1.91.1
```

## Helper Scripts

All helper scripts now automatically load the Cargo environment:

- `./scripts/setup-dev.sh` - Loads cargo env automatically
- `./scripts/build-all.sh` - Loads cargo env automatically
- `./scripts/build-binaries.sh` - Loads cargo env automatically

So you can run them directly without manual setup.

## Troubleshooting

### "cargo: command not found"

**Solution 1 (Temporary):**
```bash
source "$HOME/.cargo/env"
```

**Solution 2 (Permanent):**
Add to your shell config file as shown above.

### Verify Rust Installation

```bash
ls -la $HOME/.cargo/bin/cargo
```

Should show the cargo binary. If not, reinstall Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
