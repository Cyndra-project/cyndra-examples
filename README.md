# Cyndra Examples

This is a collection of some example apps that show what you can deploy on Cyndra.

The examples in this repository, consists of *"Hello, world!"* examples of all officially supported web frameworks and how to combine them with Cyndra resources, but also fullstack templates and more.

## How to clone, run and deploy an example

To clone an example, use the `init` command of the [`cargo-cyndra`](https://docs.cyndra.rs/introduction/installation) binary and specify the git URL and optional subfolder:

```bash
cargo cyndra init --from https://github.com/cyndra-hq/cyndra-examples --subfolder axum/hello-world

### Other forms:

# GitHub prefix. Change to 'gl:' or 'bb:' for GitLab or BitBucket
cargo cyndra init --from gh:username/repository
# Also GitHub
cargo cyndra init --from username/repository

# From local folder
cargo cyndra init --from ./path/to/folder
cargo cyndra init --from ../../another/folder
cargo cyndra init --from /home/user/some/folder

# Clone into 'my-folder', and use the project name 'my-project-name'
cargo cyndra init --from username/repository --name my-project-name my-folder
```

Then, you can navigate into the folder where it was created, and use these commands to run the example locally, and to deploy it.

```bash
# Run locally
cargo cyndra run

# Start the Cyndra environment, make sure the project has a unique name
cargo cyndra project start
# Deploy to Cyndra
cargo cyndra deploy
```
