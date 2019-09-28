# Rouille - User Creation Microservice

Provides a synchronous API where every request is processed by a thread pool.

## Stack

|Microservice Framework|DB|HTTP Framework|
|:---:|:---:|:---:|
| `Rouille` | SQLite | `Rouille` |

## Running the app
```shell script
DATABASE_URL=test.db diesel migration run 
cargo run
```

### Testing the calls
```shell script
curl -d "email=user@example.com&password=password" -X POST http://localhost:8001/signup
curl -d "email=user@example.com&password=password" -X POST http://localhost:8001/signin
```