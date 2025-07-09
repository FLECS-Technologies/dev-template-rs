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
3. Rename the contained crates as you wish
    1. In the root `Cargo.toml`
    2. The directory containing the crate
    3. The `Cargo.toml` inside the crate directory
4. Check if everything builds by executing ```cargo build``` in the repository directory
5. Commit all changes (including `Cargo.lock`)

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

The following GitHub workflows exist in this repository.

### Validate pull requests

The workflow defined in [pull_request_validate.yml](.github/workflows/pull_request_validate.yml) will run automatically
on every pull request but can be manually triggered as well.

#### rustfmt

The code format of the project is enforced
using [rustfmt](https://github.com/rust-lang/rustfmt?tab=readme-ov-file#rustfmt----) without any configuration. If you
want to customize the format look [here](https://github.com/rust-lang/rustfmt?tab=readme-ov-file#configuring-rustfmt).

#### Linting

The code is linted using [clippy](https://github.com/rust-lang/rust-clippy?tab=readme-ov-file#clippy). Clippy is used
with default settings but all warnings are treated as errors and will fail the check.

#### Documentation

The code documentation is built using [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) and attached as
an artifact.

#### Testing

All automatic tests
including [doc-tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) are executed.

#### Build binaries

The project is built for the targets `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu` and
`armv7-unknown-linux-gnueabihf`. The resulting binaries are attached as artifacts.

#### Determine tag

Determines the tag of the docker image that will be used from now on. If the workflow is called from a PR the tag will
be `pr-{pr-number}` otherwise it will contain the short form commit hash and look like `commit-{commit-sha}`.

#### Docker

Creates debug docker images. See [Build image](#build-image) for more details.

#### Deploy

Deploys the previously built debug docker images. See [Deploy image](#deploy-image) for more details.

### Build image

Creates docker images for the three supported architectures and attaches them as artifacts. The build type can be
specified via the input parameter `build_type` (`debug` or `release`). The tag for the images has to be specified via
the input parameter `tag`.

### Deploy image

Deploys the previously built docker images to the GitHub registry. The tag for the resulting images and manifest has to
be specified via the input parameter `tag`.

### Release

This workflow is executed on published releases. It builds release docker images and deploys them to the GitHub
registry. The tag for the images corresponds to the git tag of the release. See [Build image](#build-image)
and [Deploy image](#deploy-image) for more details.