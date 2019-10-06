use actix::prelude::*;
use std::io;
use actix::dev::{MessageResponse, ResponseChannel};

// Basic example

struct Ping;

impl Message for Ping {
    type Result = Result<bool, io::Error>;
}

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor is alive!");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor is dead!");
    }
}

impl Handler<Ping> for MyActor {
    type Result = Result<bool, io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        println!("Ping received");
        Ok(true)
    }
}

// Custom Message response

enum Messages {
    Ping,
    Pong
}

enum Responses {
    GotPing,
    GotPong
}

pub struct MyActor2;

// This is the code that enables a user defined type to be used as a message response
impl <A, M> MessageResponse<A, M> for Responses where A: Actor, M: Message<Result = Responses> {
    fn handle<R: ResponseChannel<M>>(self, ctx: &mut <A as Actor>::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl Message for Messages {
    type Result = Responses;
}

impl Actor for MyActor2 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Actor2 is alive!");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor2 is dead!");
    }
}

impl Handler<Messages> for MyActor2 {
    type Result = Responses;

    fn handle(&mut self, msg: Messages, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Messages::Ping => Responses::GotPing,
            Messages::Pong => Responses::GotPong
        }
    }
}

fn main() {
    let sys = System::new("Example");

    let addr = MyActor.start();
    let addr2 = MyActor2.start();

    let ping_future = addr2.send(Messages::Ping);
    let pong_future = addr2.send(Messages::Pong);

    let result = addr.send(Ping);

    Arbiter::spawn(
        result
            .map(|res| match res {
                Ok(result) => println!("Got result: {}", result),
                Err(err) => println!("Got error: {}", err),
            })
            .map_err(|e| {
                println!("Actor is probably dead: {}", e);
            }),
    );

    Arbiter::spawn(
        pong_future.map(|res| {
            match res {
                Responses::GotPing => println!("Ping received"),
                Responses::GotPong => println!("Pong received"),
            }
        })
            .map_err(|e| {
                println!("Actor is probably dead {}", e);
            })
    );

    Arbiter::spawn(
        ping_future.map(|res| {
            match res {
                Responses::GotPing => println!("Ping received"),
                Responses::GotPong => println!("Pong received"),
            }
        })
            .map_err(|e| {
                println!("Actor is probably dead {}", e);
            })
    );

    sys.run();
}
