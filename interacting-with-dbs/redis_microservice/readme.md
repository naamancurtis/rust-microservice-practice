# Redis Microservice - User Sessions
## App - Example Usage
<h3> Add Sessions </h3>

```shell script
cargo run -- add 7vQ2MhnRcyYeTptp a73bbfe3-df6a-4dea-93a8-cb4ea3998a53
cargo run -- add pTySt8FI7TIqId4N 0f3688be-0efc-4744-829c-be5d177e0e1c
cargo run -- add zJx3mBRpJ9WTkwGU f985a744-6648-4d0a-af5c-0b71aecdbcba
```

<h3> Remove Sessions </h3>

```shell script
cargo run -- remove pTySt8FI7TIqId4N
```

<h3> List Sessions </h3>

```shell script
cargo run -- list
```

## Redis

```shell script
docker run -it --rm --name test-redis -p 6379:6379 redis
```