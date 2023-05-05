# Examples

Some example apps to show what you can do with cyndra.

## How to deploy the examples

To deploy the examples, check out the repository locally

```bash
git clone https://github.com/cyndra-hq/cyndra-examples.git
```

navigate to an example root folder

```bash
cd axum/hello-world
```

Pick a project name that is something unique - in cyndra,
projects are globally unique. Then run

```bash
cargo cyndra project start --name=$PROJECT_NAME
cargo cyndra deploy --name=$PROJECT_NAME
```
