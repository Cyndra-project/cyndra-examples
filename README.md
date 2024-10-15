# Cyndra Examples

This is a collection of some example apps that show what you can deploy on Cyndra.

The examples in this repository, consists of *"Hello, world!"* examples of all officially supported web frameworks and how to combine them with Cyndra resources, but also fullstack templates and more.

## How to clone, run and deploy an example

To clone an example, use the `init` command of the [CLI](https://docs.cyndra.dev/getting-started/installation) and specify the git URL and optional subfolder:

```bash
cyndra init --from https://github.com/cyndra-hq/cyndra-examples --subfolder axum/hello-world

### Other forms:

# GitHub prefix. Change to 'gl:' or 'bb:' for GitLab or BitBucket
cyndra init --from gh:username/repository
# By default, 'gh:' (GitHub) is assumed
cyndra init --from username/repository

# From local folder
cyndra init --from ./path/to/folder
cyndra init --from ../../another/folder
cyndra init --from /home/user/some/folder

# Clone into 'my-folder', and use the project name 'my-project-name'
cyndra init --from username/repository --name my-project-name my-folder
```

Then, you can navigate into the folder where it was created, and use these commands to run the example locally, and to deploy it.

```bash
# Run locally
cyndra run

# Deploy to Cyndra
cyndra deploy
```
