# Gotham Microservice - Logging

All of the other microservice frameworks have been synchronous. In order to develop async microservices there are 3 options
1. Use the `hyper` crate directly
2. Use the `Gotham` crate _(which uses the `hyper` and `tokio` crates internally)_
3. Use the `Actix-Web` framework

# Important Note:
Current implementation of `tokio-postgress` is badly out of date - as such the current code in this repo doesn't work.
It felt like a waste to implement badly out of date code that wouldn't be used in practice.