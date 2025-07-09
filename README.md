# Repository template for developing with rust

This template can be used to jumpstart and unify development in rust. The following topics are covered:

- [Quickstart](#quickstart)
- [Project structure](#project-structure)
- [Logging](#logging)
- [Error handling](#error-handling)
- [Configuration](#configuration)
- [CI/CD pipeline](#cicd-pipeline)

## Quickstart

1. Create a new repository by clicking on the `Use this template` button on the top right
   or [here](https://github.com/new?owner=Somic-Flecs-shared-space&template_name=development-template-rs&template_owner=Somic-Flecs-shared-space)
2. Clone your new repository
3. Adjust the authors and license in the root [Cargo.toml](./Cargo.toml)
4. Rename the contained crates as you wish
    1. In the root [Cargo.toml](./Cargo.toml)
    2. The directory containing the crate
    3. The `Cargo.toml` inside each crate directory
5. Set up [Variables and secrets](#variables-and-secrets)
6. Check if everything builds by executing ```cargo build``` in the repository directory
7. Commit all changes (including `Cargo.lock`)

## Project structure

This repository contains one [cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) consisting of
one library crate template_lib and one binary crate template_bin. You can add as many additional binary or library
crates to this workspace as you want. We recommend to limit the amount of code in your binary crates and instead move
most of the logic inside the library crates.

To allow for deterministic and reproducible tests and builds the `Cargo.lock` file is checked in to version control.
See [here](https://doc.rust-lang.org/cargo/faq.html#why-have-cargolock-in-version-control) for more information. For the
same reason we pass `--locked` to cargo in our workflows.

## Logging

This project uses [tracing](https://github.com/tokio-rs/tracing)
and [tracing-subscriber](https://github.com/tokio-rs/tracing/tree/master/tracing-subscriber) from
the [tokio ecosystem](https://github.com/tokio-rs) for logging.

The default tracing subscriber is used which prints all logging to std::out. There are other subscribers
available [tracing-appender](https://github.com/tokio-rs/tracing/tree/master/tracing-appender) for example which can log
to files. It is also possible to implement a custom subscriber for more advanced scenarios. You can add as many
subscribers as you want.

A `tracing_subscriber::layer::Filter` controls which logs are passed to the subscribers. Currently,
`tracing_subscriber::filter::env::EnvFilter` is used which can be constructed from a string. In this project this string
is taken from the environment variable `RUST_LOG`, the [config](#configuration) or a default constant (
`template_lib::tracing::DEFAULT_TRACING_FILTER`) in that order.

## Error handling

This project uses the crates [anyhow](https://github.com/dtolnay/anyhow)
and [thiserror](https://github.com/dtolnay/thiserror) to simplify error handling. The very general anyhow::Result is
used in the binary crate where it does not really matter which error occurred. For the library crate custom errors are
created with thiserror to allow handling each error scenario differently. We would recommend to follow this pattern for
additional library or binary crates added to this workspace.

## Configuration

The template project uses a `config.json` file for configuration. It is expected to be in the working directory. If the
file is not present a default config is used. The config is initialized once at the start of the program and can not be
changed during runtime.

You can change the path by changing `CONFIG_PATH` in [main.rs](./template_bin/src/main.rs) of
template_bin. You can change the configurable options by changing the
struct template_lib::config::Config in [template_lib/src/config/mod.rs](./template_lib/src/config/mod.rs).

## CI/CD pipeline

### Variables and secrets

The CI/CD pipeline expects certain variables and secrets to build and deploy your application. If you only want to use
the GitHub container registry for your container images, you do not need any secrets and just the `APP_NAME` variable,
but need to look at [Deploy to GitHub container registry](#deploy-to-github-container-registry).

#### Secrets

`DOCKER_REGISTRY_USER` and `DOCKER_REGISTRY_PASSWORD` define the username and password needed to authenticate with your
container registry.

Take a look at
the [GitHub Docs](https://docs.github.com/en/actions/how-tos/security-for-github-actions/security-guides/using-secrets-in-github-actions)
for more information.

#### Variables

`DOCKER_REGISTRY` defines the container registry the images will be uploaded to.
`DOCKER_REGISTRY_NAMESPACE` defines the namespace the images will be uploaded to, e.g. `apps`. You only have to defines
this variable if you want to deploy to a namespace.
`APP_NAME` defines the name of you application, it will be used for the image name.

Take a look at
the [GitHub Docs](https://docs.github.com/en/actions/how-tos/writing-workflows/choosing-what-your-workflow-does/store-information-in-variables)
for more information.

### Deploy to GitHub container registry

If you want to deploy to the GitHub container registry you have to make some adjustments:

- Remove the if condition in [release_ghcr.yml](.github/workflows/release_ghcr.yml)
- Set a correct value for environment variables `APP_NAME` and `REGISTRY_NAMESPACE`
  in [release_ghcr.yml](.github/workflows/release_ghcr.yml)
- Remove the if condition in the `docker_ghrc` job
  in [pull_request_validate.yml](.github/workflows/pull_request_validate.yml)
- Set a correct value for inputs `app_name` and `registry_namespace` in the `docker_ghrc` job
  in [pull_request_validate.yml](.github/workflows/pull_request_validate.yml)
- If you do not want to use another container registry delete [release.yml](.github/workflows/release_ghcr.yml) and the
  job `docker` in [pull_request_validate.yml](.github/workflows/pull_request_validate.yml)

### Workflows

#### Validate pull requests

The workflow defined in [pull_request_validate.yml](.github/workflows/pull_request_validate.yml) will run automatically
on every pull request but can be manually triggered as well.

##### rustfmt

The code format of the project is enforced
using [rustfmt](https://github.com/rust-lang/rustfmt?tab=readme-ov-file#rustfmt----) without any configuration. If you
want to customize the format look [here](https://github.com/rust-lang/rustfmt?tab=readme-ov-file#configuring-rustfmt).

##### Linting

The code is linted using [clippy](https://github.com/rust-lang/rust-clippy?tab=readme-ov-file#clippy). Clippy is used
with default settings but all warnings are treated as errors and will fail the check.

##### Documentation

The code documentation is built using [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) and attached as
an artifact.

##### Testing

All automatic tests
including [doc-tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) are executed.

##### Build binaries

The project is built for the targets `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` and
`armv7-unknown-linux-gnueabihf`. The resulting binaries are attached as artifacts.

##### Determine tag

Determines the tag of the docker image that will be used from now on. If the workflow is called from a PR the tag will
be `pr-{pr-number}` otherwise it will contain the short form commit hash and look like `commit-{commit-sha}`.

##### Docker

Creates debug docker images. See [Build image](#build-image) for more details.

#### Build image

The workflow defined in [build_image.yml](.github/workflows/build_image.yml) builds docker images for the three
supported architectures and attaches them as artifacts. Look at the description of the inputs for more information.

#### Deploy image

The workflow defined in [deploy_image.yml](.github/workflows/deploy_image.yml) deploys the previously built docker
images to the GitHub registry. Look at the description of the inputs for more information.

#### Release

The workflow defined in [release.yml](.github/workflows/release.yml) is executed on published releases. It builds
release docker images and deploys them to the container registry defined
via [Variables and secrets](#variables-and-secrets). The tag for the images corresponds to the git tag of the release.
See [Build image](#build-image) and [Deploy image](#deploy-image) for more details.
The workflow defined in [release_ghcr.yml](.github/workflows/release_ghcr.yml) does the same but for the GitHub
container registry. See [Deploy to GitHub container registry](#deploy-to-github-container-registry) for more
information.