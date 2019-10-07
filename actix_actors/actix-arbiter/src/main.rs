extern crate actix;
extern crate futures;
use actix::prelude::*;
use futures::Future;

/*
    - Arbiter -

    Arbiters provide an asynchronous execution context for Actors. Where an actor contains a
    Context that defines its Actor specific execution state, Arbiters host the environment
    where an actor runs.

    As a result Arbiters perform a number of functions. Most notably, they are able to spawn a
    new OS thread, run an event loop, spawn tasks asynchronously on that event loop, and act as
    helpers for asynchronous tasks.

    - Starting an Arbiter is generally done with `System::new()`

    While it only uses one thread, it uses the very efficient event loop pattern which works
    well for asynchronous events. To handle synchronous, CPU-bound tasks, it's better to avoid
    blocking the event loop and instead offload the computation to other threads. For this
    usecase, read the next section and consider using SyncArbiter.

    - The Event Loop

    One Arbiter is in control of one thread with one event pool. When an Arbiter spawns a task
    (via Arbiter::spawn, Context<Actor>::run_later, or similar constructs), the Arbiter queues
    the task for execution on that task queue. When you think Arbiter, you can think
    "single-threaded event loop".

    Actix in general does support concurrency, but normal Arbiters (not SyncArbiters) do not.
    To use Actix in a concurrent way, you can spin up multiple Arbiters using Arbiter::new,
    ArbiterBuilder, or Arbiter::start.

    When you create a new Arbiter, this creates a new execution context for Actors. The new thread
    is available to add new Actors to it, but Actors cannot freely move between Arbiters: they are
    tied to the Arbiter they were spawned in. However, Actors on different Arbiters can still
    communicate with each other using the normal Addr/Recipient methods. The method of passing
    messages is agnostic to whether the Actors are running on the same or different Arbiters.
*/

// Using an Arbiter to resolve async events

struct SumActor {}

impl Actor for SumActor {
    type Context = Context<Self>;
}

struct Value(usize, usize);

impl Message for Value {
    type Result = usize;
}

impl Handler<Value> for SumActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, ctx: &mut Self::Context) -> Self::Result {
        msg.0 + msg.1
    }
}

struct DisplayActor {}

impl Actor for DisplayActor {
    type Context = Context<Self>;
}

struct Display(usize);

impl Message for Display {
    type Result = ();
}

impl Handler<Display> for DisplayActor {
    type Result = ();

    fn handle(&mut self, msg: Display, ctx: &mut Self::Context) -> Self::Result {
        println!("Got {:?}", msg.0);
    }
}

fn main() {
    let system = System::new("single-arbiter-example");

    // Actor::start() spawns actors on the current Arbiter
    let sum_addr = SumActor {}.start();
    let dis_addr = DisplayActor {}.start();

    // Define execution flow

    // `Addr::send()` responds with a request, which implements `Future` when awaited/mapped,
    // will resolve to `Result<usize, MailboxError>`
    let execution = sum_addr
        .send(Value(6, 7))
        // map_err turns `Future<usize, MailboxError> into Future<usize, ()>
        .map_err(|e| {
            eprintln!("Encountered error: {:?}", e);
        })
        // Assuming the send was successful, chain another computation onto the future
        .and_then(move |res| {
            // `res` is now a usize returned from the SumActor
            // Once the future is complete we can then send it to the `DisplayActor`
            dis_addr.send(Display(res)).map(move |_| ()).map_err(|_| ())
        })
        .map(move |_| {
            // We only want to do 1 computation in this example, so we shut down the
            // system, which will stop any Arbiters within it (and subsequently all
            //actors within it)
            System::current().stop();
        });

    // Spawn the future onto the current Arbiter
    Arbiter::spawn(execution);

    system.run();
}
