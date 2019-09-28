extern crate crypto;
#[macro_use]
extern crate diesel;
extern crate env_logger;
extern crate failure;
extern crate log;
#[macro_use]
extern crate rouille;
extern crate serde_derive;

use crypto::pbkdf2::{pbkdf2_check, pbkdf2_simple};
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use failure::{format_err, Error};
use log::debug;
use rouille::{router, Request, Response};
use serde_derive::Serialize;

mod models;
mod schema;

#[derive(Serialize)]
struct UserId {
    id: String,
}

fn main() {
    env_logger::init();
    let manager = ConnectionManager::<SqliteConnection>::new("test.db");
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
    rouille::start_server("127.0.0.1:8001", move |request| {
        match handler(&request, &pool) {
            Ok(response) => response,
            Err(err) => Response::text(err.to_string()).with_status_code(500),
        }
    })
}

// Rouille automatically maps these to the routes provided it's in a
// router! macro block
fn handler(
    request: &Request,
    pool: &Pool<ConnectionManager<SqliteConnection>>,
) -> Result<Response, Error> {
    debug!("Request: {:?}", request);

    let response = router!(request,
        (GET) (/) => {
            Response::text("Users Microservice")
        },
        (POST) (/signup) => {

            let data = post_input!(request, {
                email: String,
                password: String,
            })?;

            let user_email = data.email.trim().to_lowercase();
            let user_password = pbkdf2_simple(&data.password, 12345)?;

            {
                use self::schema::users::dsl::*;
                let conn = pool.get()?;

                let user_exists: bool = select(
                    exists(
                        users.filter(
                            email.eq(user_email.clone())
                        ))
                    ).get_result(&conn)?;

                if !user_exists {
                    let uuid = format!("{}", uuid::Uuid::new_v4());
                    let new_user = models::NewUser {
                        id: &uuid,
                        email: &user_email,
                        password: &user_password
                    };

                    diesel::insert_into(schema::users::table)
                        .values(&new_user)
                        .execute(&conn)?;

                    Response::json(&())
                } else {
                    Response::text(format!("User {} already exists", data.email))
                        .with_status_code(400)
                }
            }
        },
        (POST) (/signin) => {

            let data = post_input!(request, {
                email: String,
                password: String,
            })?;

            let user_email = data.email;
            let user_password = data.password;

            {
                use self::schema::users::dsl::*;
                let conn = pool.get()?;
                let user = users.filter(email.eq(user_email))
                    .first::<models::User>(&conn)?;
                let valid = pbkdf2_check(&user_password, &user.password)
                    .map_err(|err| format_err!("pass check error: {}", err))?;

                if valid {
                    let user_id = UserId {
                        id: user.id
                    };
                    Response::json(&user_id)
                        .with_status_code(200)
                } else {
                    Response::text("access denied")
                        .with_status_code(403)
                }
            }
        },
        _ => {
            Response::empty_404()
        }
    );

    Ok(response)
}
