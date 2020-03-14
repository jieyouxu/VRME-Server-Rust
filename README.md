# Virtual Reality Meeting Environment Backend Server

## Configuration

Run the application in either development mode or production mode by specifying
`RUN_MODE`:

- **Production** mode:

    ```bash
    RUN_MODE=production cargo run --release
    ```

- **Development** mode:

    ```bash
    RUN_MODE=development cargo run --release
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
