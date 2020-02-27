# VRME-Server

![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/jieyouxu/VRME-Server-Rust?logo=GitHub&logoColor=lightblue&style=flat-square)
![GitHub](https://img.shields.io/github/license/jieyouxu/VRME-Server-Rust?style=flat-square)

VRME server implementation in Rust.

## Configuration

Copy the `config.example.toml` into `config.toml` to customize the server
settings.

```bash
$ cp config.example.toml config.toml
```

## API Documentation

Available under `docs/` in markdown format, or view the rendered version at
[jieyouxu/VRME-Server-Rust](https://jieyouxu.github.io/VRME-Server-Rust/).

## Logging

We utilize [rust-lang/log](https://github.com/rust-lang/log) and
[seanmonstar/pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger)
to handle logging.

To specify different logging levels, the `RUST_LOG` environment variable should
be configured to one of the options below (ordered by decreasing verbosity):

1. `RUST_LOG=trace`.
2. `RUST_LOG=debug`.
3. `RUST_LOG=info`
4. `RUST_LOG=warn`
5. `RUST_LOG=error`

Or leave `RUST_LOG` empty to disable logging output.

For example:

```bash
$ RUST_LOG=debug cargo run
```

