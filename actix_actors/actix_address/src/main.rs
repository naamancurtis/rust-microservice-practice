#[macro_user] extern crate actix;
use actix::prelude::*;


/*
    Actors can't be referenced directly, instead they can only be referenced
    through their address. There are several ways to get this address

    1. The helper methods for starting an actor
*/

/*
    - Messages -

   To send a message to an Actor, the `Addr` object needs to be used, there are 3
   main ways to do this

   1. Addr::do_send() - ignores any errors, bypasses mailbox limits (is silently dropped if the mailbox is closed). No result is returned
   2. Addr::try_send() - Tries to send the message immediately, returns a SendError if there is an error
   3. Addr::send() - Returns a future object that resolves to a result of a message

*/

/*
    - Recipient -

    Recipient is a specialized version of an address that supports only one type of message.
    It can be used in case the message needs to be sent to a different type of actor.
     A recipient object can be created from an address with `Addr::recipient()`

     An example use case is a subscription system (example below)
*/


#[derive(Message)]
struct Signal(usize);

#[derive(Message)]
struct Subscribe(pub Recipient<Signal>);

struct ProcessSignals {
    subscribers: Vec<Recipient<Signal>>
}

impl Actor for ProcessSignals {
    type Context = Context<Self>;
}

impl ProcessSignals {
    fn send_signal(&mut self, sig: usize) {
        for subscriber in &self.subscribers {
            subscriber.do_send(Signal(sig));
        }
    }
}

impl Handler<Subscribe> for ProcessSignals {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        self.subscribers.push(msg.0)
    }
}

/*
    - Context -

    All actors maintain their internal execution context (state)
    - This allows it to determine itself
*/

// Mailbox

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(1); // Default is 16
    }
}

// Address
struct WhoAmI;

impl Message for WhoAmI {
    type Result = Result<actix::Addr<MyActor>, ()>;
}

impl Handler<WhoAmI> for MyActor {
    type Result = Result<actix::Addr<MyActor>, ()>;

    fn handle(&mut self, msg: WhoAmI, ctx: &mut Context<Self>) -> Self::Result {
        Ok(ctx.address())
    }
}


fn main() {
    let system = System::new("test");
    let addr = MyActor.start();
    let whoami = addr.do_send(WhoAmI {});
}