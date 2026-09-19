# Cliva

A Rust toolkit for building reliable, polished, and developer-friendly command-line applications.

Cliva is a collection of focused Rust libraries designed to make CLI development faster, cleaner, and less repetitive.

The project is organized as a Cargo workspace containing independently usable crates that share the same repository and development environment.

## Crates

### `cliva`

The core CLI development library.

It provides the building blocks required to create command-line applications, including command definitions, argument handling, options, flags, subcommands, parsing, and related CLI functionality.

**Crate:** `cliva`

```toml
[dependencies]
cliva = "0.1"
```

### `cliva-io`

A standalone terminal input and output toolkit for Rust CLI applications.

It focuses on making terminal interaction easier and more consistent, providing utilities for things such as formatted output, user input, prompts, status messages, and other terminal UI functionality.

cliva-io is independent from cliva and can be used on its own.

**Crate:** `cliva-io`

```toml
[dependencies]
cliva-io = "0.1"
```

## Design

Cliva is intentionally split into focused libraries.

```text
                    Cliva Workspace
                          |
             +------------+------------+
             |                         |
           cliva                   cliva-io
             |                         |
      CLI Development              Terminal I/O
             |                         |
      +------+-------+          +------+-------+
      |              |          |              |
   Commands       Parsing     Input          Output
   Arguments      Routing     Prompts        Formatting
   Options        Macros      Selection      Status
   Flags                     Confirmation     Tables
```

The crates are independently usable. Using one does not require using the other.

This allows developers to choose only the functionality their CLI actually needs.


## Why Cliva?

Building a CLI should not require repeatedly solving the same problems.

cliva aims to provide well-designed primitives for common CLI development tasks while keeping the APIs simple, modular, and predictable.

The project focuses on:
- Less boilerplate
- Clear and expressive APIs
- Independent components
- Consistent terminal interaction
- Developer-friendly abstractions
- Reliable CLI behavior
- Minimal unnecessary dependencies


## Example

A CLI can combine both crates when needed:

```rust
use cliva::Command;
use cliva_io::output;

fn main() {
    // CLI logic
    // ...

    output::success("Operation completed successfully.");
}
```

>
> The exact API is currently under development and may change before the first stable release.
>

## Workspace

The repository is a Cargo workspace containing independently versioned crates.

```text
cliva/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
│
├── cliva/
│   ├── Cargo.toml
│   └── src/
│
└── cliva-io/
    ├── Cargo.toml
    └── src/
```

• Each crate has its own version and release cycle while sharing common workspace metadata such as the repository, license, authorship, Rust edition, and minimum supported Rust version.


## Status

>
> Cliva is currently **under active development**.
> 
> APIs **may change** as the project evolves. Until the crates reach a stable release, users should expect breaking changes between versions.
>


## Development

Clone the repository:

```bash
git clone https://github.com/theNexmentProject/cliva.git
cd cliva
```

Build the entire workspace:

```bash
cargo build --workspace
```

Run tests:

```bash
cargo test --workspace
```

Check the workspace:

```bash
cargo check --workspace
```

Format the code:

```bash
cargo fmt --all
```

Run Clippy:

```bash
cargo clippy --workspace --all-targets --all-features
```

## Documentation

Documentation will be available through docs.rs once the crates are published.

[cliva](https://docs.rs/cliva) and [cliva-io](https://docs.rs/cliva-io)

Visit [website](https://app.nexment.in/libraries/cliva) for more details.


## Contributing

> Contributions are welcome.

Before submitting a pull request:

- Format the code with cargo fmt.
- Run cargo check --workspace.
- Run cargo test --workspace.
- Run Clippy and address relevant warnings.
- Keep changes focused and consistent with the project's design.

For larger changes, open an issue first to discuss the proposed design.


## Security

If you discover a security vulnerability, please report it privately.

Security contact: **admin@nexment.in**

Please do not publicly disclose security vulnerabilities before they have been reviewed.


---


## License

This project is licensed under the terms of **Apache License 2.0**.

See the [LICENSE](LICENSE) file for details.


---


This Library is made under **The Nexment Project**.

