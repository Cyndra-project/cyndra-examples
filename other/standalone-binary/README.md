# Standalone binary - run an app with Cyndra or standalone

This example shows how to separate a project's Cyndra logic from its core functionality so that two binaries can be made: one for running with `cargo cyndra run` and deploying to Cyndra, and one that can be run with `cargo run --bin ...`.

All startup logic is placed in the binary source files, while the implementation (endpoints etc) is moved to the library of the crate.

- `src/bin/cyndra.rs` is the main binary with Cyndra, run with `cargo cyndra run`. Note that the `[[bin]]` entry in `Cargo.toml` needs to have the same name as the crate. The file can have any name you want.
- `src/bin/standalone.rs` is without Cyndra, run with `cargo run --bin standalone` (you can change the name)

This example shows how to use separate logic for getting secrets (Cyndra secrets vs homemade solution), but the same approach can be applied to other resources that are initiated by Cyndra's main function.
