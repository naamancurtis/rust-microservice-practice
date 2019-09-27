# Chat App

## Setting up Diesel with Postgres

1. Pull the PostGres image and get it running
```shell script
docker run -d -p 5432:5432 --name <Image Name> -e POSTGRES_PASSWORD=<password> postgres

# Can also use
-v <LocalDirectory>:/var/lib/postgresql/data
# To mount the volumes

# It's also possible to pass other environment variables such as
# POSTGRES_USER and POSTGRESDB
```

2. Once the Image is running, create the new cargo project and add the dependencies
    - Probably a good idea to use `dotenv` alongside diesel
    
3. Run the following command or add it as an environment variable _(Or add it to the subsequent diesel command with `--database-path`)_

```shell script
echo DATABASE_URL=postgres://<username>:<password>@localhost/<database name> > .env
```
This adds it to the `.env` file, but essentially you need to tell Diesel how to access the database

4. Run `diesel setup` - this should generate a load of files

5. Create the migrations `diesel migration generate <name>` and add the SQL to them

6. Run `diesel migration run` in order to auto-gen the rust code

7. This library can then be ran and tested with `diesel migration run && cargo test`
    - Note you're only able to do this once as there will be a duplicate error on the key 


## Useful Postgres Commands
- Get into the container
```shell script
docker docker exec -it test-postgres bash
```

- Use `psql`
```shell script
psql -U postgres <database name>
```

- List tables 
```shell script
\dt
```

- Select a table
```sql
SELECT * FROM <tablename>;
# Make sure you terminate with ;
```