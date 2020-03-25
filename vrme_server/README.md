# Development Instructions

## Database Setup

1. Create user:

```bash
createuser -P <USERNAME>
```

2. Create database:

```bash
psql -f src/database/setup/init_accounts.sql
```

3. Edit configuration under `config/` and run the server:

```bash
cargo run
```
