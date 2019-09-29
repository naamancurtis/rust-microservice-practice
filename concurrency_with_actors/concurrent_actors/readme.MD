# Actors in Microservices

To develop a microservice using actors, you should split your service into tasks that solves different kinds of work. For example, you can use a separate actor for every incoming connection or database interaction, and even as a supervisor to control other actors. Every actor is an asynchronous task that is executed in a reactor. 

The benefits of this approach are as follows: 

- It's simpler to write separate actors than a bulk of functions 
- Actors can fail and respawn 
- You can reuse actors 

One important benefit of using actors is reliability, because every actor can be failed and respawned, so you don't need a long recovery code to handle failures. It doesn't mean your code can call the panic! macro everywhere, but this does mean that you can consider actors as short life cycle tasks that work concurrently on small tasks. 

If you design actors well, you also gain great performance, because interaction with messages helps you to split work into short reactions, which won't block the reactor for a long time. Also, your source code becomes more structured.

## The Actix Framework 
The Actix framework provides an actors model for Rust, based on the futures crate and some asynchronous code to allow actors to work concurrently with minimal resources needed.