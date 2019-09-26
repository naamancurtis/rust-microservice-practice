#[macro_use]
extern crate diesel;

use self::models::*;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand,
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use failure::Error;
use uuid::Uuid;

pub mod models;
pub mod schema;

const CMD_ADD: &str = "add";
const CMD_LIST: &str = "list";

fn main() -> Result<(), Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("db")
                .value_name("FILE")
                .help("Sets a file name of a database")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name(CMD_ADD)
                .about("Add user to the table")
                .arg(
                    Arg::with_name("NAME")
                        .help("Sets the name of a user")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("EMAIL")
                        .help("Sets the email of a user")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(SubCommand::with_name(CMD_LIST).about("Prints a list of users"))
        .get_matches();

    let path = matches.value_of("database").unwrap_or("test.db");
    let manager = ConnectionManager::<SqliteConnection>::new(path);
    let pool = Pool::new(manager)?;

    match matches.subcommand() {
        (CMD_ADD, Some(matches)) => {
            let conn = pool.get()?;
            let parsed_name = matches.value_of("NAME").unwrap();
            let parsed_email = matches.value_of("EMAIL").unwrap();
            let new_uuid = format!("{}", Uuid::new_v4());
            let new_user: NewUser = NewUser {
                id: new_uuid.as_ref(),
                name: parsed_name,
                email: parsed_email,
            };

            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .execute(&conn)?;
        }
        (CMD_LIST, _) => {
            use self::schema::users::dsl::*;
            let conn = pool.get()?;
            let items = users
                .filter(email.like("%@example.com"))
                .limit(10)
                .load::<User>(&conn)?;
            for user in items {
                println!("{:?}", user);
            }
        }
        _ => {
            matches.usage();
        }
    }

    Ok(())
}
