# Virtual Reality Meeting Environment Backend Server

![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/jieyouxu/VRME-Server-Rust?logo=GitHub&style=flat-square)
![GitHub](https://img.shields.io/github/license/jieyouxu/VRME-Server-Rust?style=flat-square)
![GitHub deployments](https://img.shields.io/github/deployments/jieyouxu/VRME-Server-Rust/github-pages?label=documentation%20deployment&logo=GitHub&style=flat-square)

## Development

The project is written in [Rust](https://github.com/rust-lang/rust). It is
written in Rust Edition `2018`, using the latest stable release.

- We use [Cargo](https://github.com/rust-lang/cargo/) for the package manager.
- We use [rustup](https://github.com/rust-lang/rustup/) for managing the tool
  chain.
- We use [mdBook](https://github.com/rust-lang/mdBook) for API endpoint
  documentation.
- We use [rustfmt](https://github.com/rust-lang/rustfmt) for formatting Rust
  source code.

The actual Rust project is under `vrme_server`, so start by changing the working
directory into `vrme_server`:

```bash
cd vrme_server
```

All of the instructions below assume that you are under the `vrme_server`
directory already.

### Code Documentation

If you want to generate and render code documentation for `vrme_server`, run

```bash
cargo doc
```

And the docs will be generated under `target/doc`.

- If you wish to open the documentation in the browser directly, run instead
  (the browser executable pointed to by `$BROWSER`):

   ```bash
   cargo doc --open
   ```

- If you don't want to generate docs for dependency crates, specify the
  `--no-deps` flag to `cargo doc`

   ```bash
   cargo doc --no-deps --open
   ```

## API Documentation

Available at
[VRME-API-Documentation](https://github.com/jieyouxu/VRME-API-Documentation).

## Logging Level

To specify the logging level, provide `LOG` with one of:

| Verbosity     | `LOG=`  |
|---------------|---------|
| Most verbose  | `TRACE` |
|               | `DEBUG` |
|               | `INFO`  |
|               | `WARN`  |
| Least verbose | `ERROR` |

`LOG=INFO` is the default logging level.

Example log level setting:

```
LOG=WARN RUN_MODE=production cargo run --release
```

## Configuration

Run the application in either development mode or production mode by specifying
`RUN_MODE`:

- **Production** mode:

    ```bash
    RUN_MODE=production cargo run --release
    ```

    *The `--release` flag passed to `cargo` enables `-O3` optimization by
    default which is suitable for production mode, but erases useful debugging
    information for development.*

- **Development** mode:

    ```bash
    RUN_MODE=development cargo run
    ```

The server will:

1. Read from a base (shared) configuration file:

    ```bash
    # Copy example default configuration
    cp config/default.example.toml config/default.toml
    ```

2. Read from a `RUN_MODE`-dependent configuration file.

    Change `$RUN_MODE` below to either `production` or `development`.

    ```bash
    # Copy example $RUN_MODE-dependent configuration
    cp config/$RUN_MODE.example.toml config/$RUN_MODE.toml
    ```

3. Read from environment variables that have the same key name as the ones in
   the configuration file. The environment variables need to be *prefixed* by
   `APP_`. When the configuration is nested, such as `server.hostname`, use the
   separator `__` to replace the dot as the name of the environment variable.

   *Example: `server.hostname` is overriden by `APP_SERVER__HOSTNAME`*.

   ```bash
   # Override `server.hostname`
   APP_SERVER__HOSTNAME=127.0.0.1 \
       RUN_MODE=development \
       cargo run
   ```

Environment variables take precedence over `RUN_MODE`-specific configuration
files, which in turn take precedence over the default shared configuration
file.

See the `src/settings` module for the most accurate configuration options.
