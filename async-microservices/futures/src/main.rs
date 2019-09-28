extern crate failure;
extern crate futures;
extern crate tokio;

use failure::Error;
use futures::sync::{mpsc, oneshot};
use futures::{future, stream, Future, IntoFuture, Sink, Stream};
use tokio::codec::LinesCodec;
use tokio::io;
use tokio::net::{UdpFramed, UdpSocket};

fn main() {
    /*
       You have to use a reactor to get the result for types that implement the Future trait.
       You can't get the result immediately and use it in the next expression; instead, you
       have to create chains of futures or streams to get the appropriate result.

       Stream <Trait> represents a sequence of deferred items (similar to an Iterator)
       It uses the .poll() method to get the next `Item` (or Error)
       It's possible to convert - Future <-> Stream

       Streams should be manipulated through chaining transformations on them

       Sinks are used to send data from the receiver back to the source (opposite of streams)
    */

    // Channels for sending multiple messages
    /*
        Single-Producer Single-Consumer

        - This means a single Sender and single Receiver (neither of which can be cloned)
        - If you require this queue you need to use the extern crate: bounded-spsc-queue
    */
    /*
        Multi-Producer Single-Consumer

        - Implementations in both the std and futures crates
        - The sender can be cloned, but the receiver can't
    */
    /*
        Multi-Producer Multi-Consumer

        - Implementations in the crossbeam-channel crate
        - You can't predict which thread will get a specific message
    */

    multiple();
    single();
    alt_upd_echo();
}

// The most common is the MPSC channel - below is a futures example
fn multiple() {
    let (tx_sink, rx_stream) = mpsc::channel::<u8>(8);
    let receiver = rx_stream
        .fold(0, |acc, value| future::ok(acc + value)) // Waits for the channel to be closed, then adds all the totals
        .map(|x| {
            println!("Calculated: {}", x);
        });

    let send_1 = tx_sink.clone().send(1);
    let send_2 = tx_sink.clone().send(2);
    let send_3 = tx_sink.clone().send(3);

    let execute_all = future::join_all(vec![
        to_box(receiver),
        to_box(send_1),
        to_box(send_2),
        to_box(send_3),
    ])
    .map(drop);

    drop(tx_sink); // To close the sender, we have to drop it
                   // If we don't the channel will remain open and tokio::run will never finish

    tokio::run(execute_all);
}

fn to_box<T>(fut: T) -> Box<dyn Future<Item = (), Error = ()> + Send>
where
    T: IntoFuture,
    T::Future: Send + 'static,
    T::Item: 'static,
    T::Error: 'static,
{
    let fut = fut.into_future().map(drop).map_err(drop);
    Box::new(fut)
}

/*
    Oneshot is a module that has a channel which contains a single message which is
    consumed instantly and then closed
*/
fn single() {
    let (tx_sender, rx_future) = oneshot::channel::<u8>();

    let receiver = rx_future.map(|x| {
        println!("Received: {}", x);
    });

    let sender = tx_sender.send(8);

    let execute_all = future::join_all(vec![to_box(receiver), to_box(sender)]).map(drop);

    tokio::run(execute_all);
}

// Using channels to use Sink in multiple places
fn alt_upd_echo() -> Result<(), Error> {
    let from = "0.0.0.0:12345".parse()?;
    let socket = UdpSocket::bind(&from)?;
    let framed = UdpFramed::new(socket, LinesCodec::new());

    let (sink, stream) = framed.split();
    let (tx, rx) = mpsc::channel(16);

    let rx = rx
        .map_err(|_| other("can't take message"))
        .fold(sink, |sink, frame| sink.send(frame));

    let process = stream
        .and_then(move |args| tx.clone().send(args).map(drop).map_err(other))
        .collect();

    let execute_all = future::join_all(vec![to_box(rx), to_box(process)]).map(drop);

    Ok(tokio::run(execute_all))
}

fn other<E>(err: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, err)
}

/*
    Running Async Tasks
    1. Run via blocking - need to be very careful with this, not for use with async code
    2. Run via an Executor
*/

// An executor allows you to perform multiple tasks in a single thread
fn send_spawn() {
    let (tx_sink, rx_stream) = mpsc::channel::<u8>(8);

    let receiver = rx_stream
        .fold(0, |acc, value| {
            println!("Received: {}", value);
            future::ok(acc + value)
        })
        .map(drop);

    let spawner = stream::iter_ok::<_, ()>(1u8..11u8)
        .map(move |x| {
            let fut = tx_sink.clone().send(x).map(drop).map_err(drop);
            tokio::spawn(fut);
        })
        .collect();

    let execute_all = future::join_all(vec![to_box(spawner), to_box(receiver)]).map(drop);

    tokio::run(execute_all);
}
