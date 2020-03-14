# Virtual Reality Meeting Environment Backend Server

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

### Code Documentation

If you want to generate and render code documentation for `vrme_server`, run

```bash
cargo doc
```

And the docs will be generated under `target/doc`.

- If you wish to open the documentation in the browser directly, run instead

    ```bash
    cargo doc --open
    ```

## API Documentation

Source under `docs/`.

View the live deployment at
[VRME-Server-Rust](https://jieyouxu.github.io/VRME-Server-Rust/).

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
   `APP_`.

   *Example: `server.hostname` is overriden by `APP_SERVER_HOSTNAME`*.

   ```bash
   # Override `server.hostname`
   APP_SERVER_HOSTNAME=127.0.0.1 \
       RUN_MODE=development \
       cargo run
   ```

Environment variables take precedence over `RUN_MODE`-specific configuration
files, which in turn take precedence over the default shared configuration
file.

See the `src/config` module for the most accurate configuration options.
