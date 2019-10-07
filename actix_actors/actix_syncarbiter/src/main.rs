use actix::prelude::*;

/*
    When you normally run Actors, there are multiple Actors running on the System's Arbiter
    thread, using its event loop. However for CPU bound workloads, or highly concurrent
    workloads, you may wish to have an Actor running multiple instances in parallel.

    This is what a SyncArbiter provides - the ability to launch multiple instances of an
    Actor on a pool of OS threads.

    It's important to note a SyncArbiter can only host a single type of Actor. This means
    you need to create a SyncArbiter for each type of Actor you want to run in this manner.
*/

struct MySyncActor;

impl Actor for MySyncActor {
    // Note how the context is now SyncContext
    type Context = SyncContext<Self>;
}

fn main() {
    // We can only control the number of threads at SyncArbiter creation time, we can't add or
    // remove threads later

    System::new("test");
    let addr = SyncArbiter::start(2, || MySyncActor);
}
