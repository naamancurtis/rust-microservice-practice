# Actors

Threads and thread pools are good ways to utilize more resources of a server, but it's a tedious programming style. You have to think about a lot of details: sending and receiving messages, load distribution, and respawning failed threads. 

There's another approach to run tasks concurrently: actors. The actors model is a computational model that uses computational primitives called actors. They work in parallel and interact with each other by passing messages. It's a more flexible approach than using threads or pools, because you delegate every complex task to a separate actor that receives messages and return results to any entity that sent a request to an actor. Your code becomes well structured and you can even reuse actors for different projects.

Although generally built ontop of the `tokio` and `futures` crates, implementing Actors can be an effective way to abstract away the complexity of these crates.

A good example of a crate with this abstraction is the `Actix` crate.

## Testing the Microservice
```shell script
curl --request POST \
     --data-binary "@../media/image.jpg" \
     --output "files/resized.jpg" \
     "http://localhost:8080/resize?width=100&height=100"
```