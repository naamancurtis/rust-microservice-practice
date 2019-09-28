# Postgres Miniature Users Microservice

## Connection Pool

### App
```shell script
cargo run -- --help # To see options

cargo run -- [OPTIONS] <SUBCOMMAND>

SUBCOMMANDs:
  create # Creates user table
  add <user-name> <user-email> # Adds a user
  list # Prints a list of all users
  import < <file-name># Imports users from the csv - note < is mandatory between the subcommand and file name
 
```

### Running Postgres in Docker
1. Start the Postgres Container
```shell script
docker run -it --rm --name test-pg -p 5432:5432 postgres
```

2. If you need to connect to the container, open a new terminal
```shell script
docker exec -it test-pg psql -U postgres

# check the connection
\c <database-name>

# View the tables
\dt
```
