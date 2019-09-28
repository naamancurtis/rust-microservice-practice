# Rocket Microservices - Content
Rocket requires the nightly build.
```shell script
rustup default nightly

# Given the regular updates of this, it's probably worth regularly running
rustup update

# This can be revered with:
rustup default stable
```

## Rocket.toml
To use the framework, you need to have a `Rocket.toml` config file.

You can override any parameter using environment variables. For example, if we need to set the port parameter to 80, we can run a microservice with a command:
`ROCKET_PORT=3721 cargo run`

The Rocket framework also supports three different types of environment: development, staging, and production. It allows you to have three configurations in one. Add an extra section in addition to the global section and run a microservice with the corresponding mode: `ROCKET_ENV=staging cargo run`

## Testing
```shell script
curl -d 'uid=user_id&text="this is a comment"' -X POST http://localhost:8003/new_comment
curl http://localhost:8003/list
```