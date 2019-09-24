# MongoDB Microservice - User Sessions
## App - Example Usage
<h3> Add Activity </h3>

```shell script
cargo run -- add 43fb507d-4cee-431a-a7eb-af31a1eeed02 "Logged In"
cargo run -- add 43fb507d-4cee-431a-a7eb-af31a1eeed02 "Added contact information"
cargo run -- add 43fb507d-4cee-431a-a7eb-af31a1eeed02 "E-mail confirmed"
```

<h3> List Activity </h3>

```shell script
cargo run -- list
```

## Redis

```shell script
docker run -it --rm --name test-mongo -p 27017:27017 mongo
