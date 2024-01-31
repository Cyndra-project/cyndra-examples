## Cyndra Cron Template
This repository is an example of how you can deploy a cronjob to [Cyndra](https://www.cyndra.rs), the Rust-native dev cloud platform.

### Usage
Make sure you have `cargo-cyndra` installed! If not, you can use `cargo install cargo-cyndra` to do so. Requires Rust 1.70+ installed.

Run the following to initialise this project:
```bash
cargo cyndra init --from cyndra-hq/cyndra-examples --subfolder cyndra-cron
```

Follow the prompt, then make any changes you want. Once done, you can deploy:
```bash
cargo cyndra deploy 

# use this if on a dirty Git branch
cargo cyndra deploy --allow-dirty
```
