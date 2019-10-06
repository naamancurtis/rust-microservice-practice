use actix::prelude::*;

struct MyActor {
    count: usize,
}

// In order to be an Actor, it must implement the Actor trait
impl Actor for MyActor {
    type Context = Context<Self>;
}

struct Ping(usize);

// Next we need to define the message that the actor accepts
// ie. Any actor that accepts a Ping message, must return a 'usize' result
impl Message for Ping {
    type Result = usize;
}

// Finally we need to declare that MyActor can handle Ping Messages
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

fn main() -> std::io::Result<()> {
    let system = System::new("test");

    // Start a new actor
    // ie. Turn the actor into a future, and result the address of the actor
    let addr = MyActor { count: 10 }.start();

    // Send a message and get the future for the result
    let res = addr.send(Ping(10));

    Arbiter::spawn(
        res.map(|res| {
            println!("Result: {}", res == 20);
        })
        .map_err(|_| {}),
    );

    system.run()
}
