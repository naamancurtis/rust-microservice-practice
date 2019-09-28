use clap::{
    crate_authors, crate_description, crate_name, crate_version,
    App, AppSettings, Arg, SubCommand,
};
use redis::{Commands, Connection, RedisError};
use r2d2_redis::RedisConnectionManager;
use std::collections::HashMap;

const SESSIONS: &str = "sessions";
const CMD_ADD: &str = "add";
const CMD_REMOVE: &str = "remove";
const CMD_LIST: &str = "list";

fn add_session(conn: &mut Connection, token: &str, uid: &str) -> Result<(), RedisError> {
    conn.hset(SESSIONS, token, uid)
}

fn remove_session(conn: &mut Connection, token: &str) -> Result<(), RedisError> {
    conn.hdel(SESSIONS, token)
}

fn list_sessions(conn: &mut Connection) -> Result<HashMap<String, String>, RedisError> {
    conn.hgetall(SESSIONS)
}

fn main() -> Result<(), failure::Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("db")
                .value_name("ADDR")
                .help("Sets the address of the db connection")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name(CMD_ADD).about("Add a session")
            .arg(Arg::with_name("TOKEN")
                .help("Sets the token of a user")
                .required(true)
                .index(1)
            )
            .arg(Arg::with_name("UID")
                .help("Sets the UID of a user")
                .required(true)
                .index(2)
            )
        )
        .subcommand(SubCommand::with_name(CMD_REMOVE).about("Remove a session")
            .arg(Arg::with_name("TOKEN")
                .help("Sets the token of a user")
                .required(true)
                .index(1)
            )
        )
        .subcommand(SubCommand::with_name(CMD_LIST).about("Print a list of all sessions"))
        .get_matches();

    let addr = matches.value_of("database")
        .unwrap_or("redis://127.0.0.1/");
    let manager = RedisConnectionManager::new(addr)?;
    let pool = r2d2::Pool::builder().build(manager)?;
    let mut conn = pool.get()?;

    match matches.subcommand() {
        (CMD_ADD, Some(matches)) => {
            let token = matches.value_of("TOKEN").unwrap();
            let uid = matches.value_of("UID").unwrap();
            add_session(&mut conn, token, uid)?;
        }
        (CMD_REMOVE, Some(matches)) => {
            let token = matches.value_of("TOKEN").unwrap();
            remove_session(&mut conn, token)?;
        }
        (CMD_LIST, _) => {
            println!("Listing Sessions");
            let sessions = list_sessions(&mut conn)?;
            for (token, uid) in sessions {
                println!("Token: {:20} UID: {:20}", token, uid);
            }
        }
        _ => { matches.usage(); }
    }

    Ok(())
}
