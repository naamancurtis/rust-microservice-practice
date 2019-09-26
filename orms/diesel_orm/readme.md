# Using ORMs in Rust - Diesel

## Installing & using Diesel
### Installing
```shell script
cargo install diesel_cli

# Note: Must have all the relevant database client libraries installed
# eg. MySQL
brew install mysql-connector-c
```

### Setting up a project
```shell script
diesel setup
```

This creates a `/migrations` folder in the project
It also auto-generates a `diesel.toml` & `src/schema.rs`

### Generating Migrations
```shell script
diesel migration generate <name>
```

This generates a pair of files:
- `up.sql` - Statements for applying migrations
- `down.sql` - Statements for reverting migrations

All migrations have to be manually written in the file

### Testing the migrations
You can then create a test database and apply the migrations with
```shell script
DATABASE_URL=test.db diesel migration run
```

## Using the App

### Adding Users
```shell script
cargo run -- add user1 user1@example.com
cargo run -- add user2 user2@example.com
cargo run -- add userx userx@xample.com
```

### Listing Users
```shell script
cargo run -- list
```