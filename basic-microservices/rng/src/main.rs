use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use dotenv::dotenv;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use log::{debug, info, trace, warn};
use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct Config {
    address: SocketAddr,
}

fn main() {
    // Read config from .env file - Crate: dotenv
    dotenv().ok();

    // Initialise the logger - Crate: log & pretty_env_logger
    pretty_env_logger::init();

    // Build the Parser for using Command Line Arguments - Crate: Clap
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("ADDRESS")
                .help("Sets an address")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config = File::open("microservice.toml")
        .and_then(|mut file| {
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            Ok(buffer)
        })
        .and_then(|buffer| {
            toml::from_str::<Config>(&buffer)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
        })
        .map_err(|err| {
            warn!("Can't read config file: {}", err);
        })
        .ok();

    info!(
        "Starting Service: {} - Version: {}",
        crate_name!(),
        crate_version!()
    );

    let addr = matches
        .value_of("address")
        .map(|s| s.to_owned()) // Priority goes to Command Line Arg
        .or(env::var("ADDRESS").ok()) // Then to Environment Var (|| .env file)
        .and_then(|addr| addr.parse().ok())
        .or(config.map(|config| config.address)) // Then to .toml config
        .or_else(|| Some(([127, 0, 0, 1], 8080).into())) // Then a default value
        .unwrap();
    debug!("Trying to bind server to address: {}", addr);

    let builder = Server::bind(&addr);

    trace!("Creating service handler..");
    let server = builder.serve(|| {
        service_fn_ok(|req| {
            trace!("Incoming request is: {:?}", req);
            let random_byte = rand::random::<u8>();
            debug!("Generated value is: {}", random_byte);
            Response::new(Body::from(random_byte.to_string()))
        })
    });

    info!("Used address: {}", server.local_addr());
    let server = server.map_err(drop);
    debug!("Running Server...");
    hyper::rt::run(server);
}
