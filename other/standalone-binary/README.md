# Standalone binary - run an app with Cyndra or standalone

This example shows how to separate a project's Cyndra logic from its core functionality so that two binaries can be made: one running with Cyndra using `cargo cyndra run` that can be deployed to Cyndra, and one that can be run with `cargo run --bin ...`.

The main idea is to have a main binary that Cyndra runs, and another binary that runs standalone.
All startup logic is placed in the binary source files, while the implementation (endpoints etc) is moved to the library of the crate.

- `src/main.rs` is the main binary with Cyndra, run with `cargo cyndra run`
- `src/standalone.rs` is without Cyndra, run with `cargo run --bin standalone` (you can change the name)

This example shows how to use separate logic for getting secrets, but the same approach can be applied to other resources that are initiated by Cyndra's main function.
