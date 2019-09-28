use failure::{format_err, Error};
use futures::{future, Future, Stream};
use gotham::handler::HandlerFuture;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::{build_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;
use gotham::state::{FromState, State};
use gotham_derive::StateData;
use hyper::header::{HeaderMap, USER_AGENT};
use hyper::{Response, StatusCode};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::reactor::Reactor;
use tokio_postgres::{Connection, TlsMode};

#[derive(Clone, StateData)]
struct ConnState {
    client: Arc<Mutex<Connection>>,
}

impl ConnState {
    fn new(client: Connection) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }
}

pub fn main() -> Result<(), Error> {
//    let mut runtime = Runtime::new()?;
    let mut reactor = Reactor::new().unwrap();

    let handshake =
        Connection::connect("postgres://postgres:mysecretpassword:5432/user_agent", TlsMode::None, &reactor.handle());
    let (mut client, connection) = runtime.block_on(handshake)?; // This blocks to enforce we have a database connection
    runtime.spawn(connection.map_err(drop));

    let execute = client.batch_execute(
        "CREATE TABLE IF NOT EXISTS agents (
            agent TEXT NOT NULL,
            timestamp TIMESTAMPZ NOT NULL DEFAULT NOW()
        );",
    );
    runtime.block_on(execute)?; // This blocks to enforce us having a valid table to use

    let state = ConnState::new(client);
    let router = router(state);

    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    gotham::start_on_executor(addr, router, runtime.executor());
    runtime
        .shutdown_on_idle()
        .wait()
        .map_err(|()| format_err!("can't wait for the runtime"))
}

fn router(state: ConnState) -> Router {
    let middleware = StateMiddleware::new(state);
    let pipeline = single_middleware(middleware);
    let (chain, piplines) = single_pipeline(pipeline);

    build_router(chain, piplines, |route| {
        route.get("/").to(register_user_agent);
    })
}

fn register_user_agent(state: State) -> Box<HandlerFuture> {
    let user_agent = HeaderMap::borrow_from(&state)
        .get(USER_AGENT)
        .map(|value| value.to_str().unwrap().to_string());

    let conn = ConnState::borrow_from(&state);
    let client_1 = conn.client.clone();
    let client_2 = conn.client.clone();

    let res = future::ok(())
        .and_then(move |_| {
            let mut client = client_1.lock().unwrap();
            client.prepare("INSERT INTO agents (agent) VALUES ($1) RETURNING agent")
        })
        .and_then(move |statement| {
            let mut client = client_2.lock().unwrap();
            client
                .query(&statement, &[&user_agent])
                .collect()
                .map(|rows| rows[0].get::<_, String>(0))
        })
        .then(|res| {
            let mut builder = Response::builder();
            let body = {
                match res {
                    Ok(value) => {
                        let value = format!("User-Agent: {}", value);
                        builder.status(StatusCode::OK);
                        value.into()
                    }
                    Err(err) => {
                        builder.status(StatusCode::INTERNAL_SERVER_ERROR);
                        err.to_string().into()
                    }
                }
            };

            let response = builder.body(body).unwrap();
            Ok((state, response))
        });

    Box::new(res)
}
