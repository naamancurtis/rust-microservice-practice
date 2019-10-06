use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use futures::Future;

use crate::errors::ServiceError;
use crate::models::{Invitation, Pool, SlimUser, User};
use crate::utils::hash_password;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

pub fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || {
        query(
            invitation_id.into_inner(),
            user_data.into_inner().password,
            pool,
        )
    })
    .then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(&user)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServiceError),
        },
    })
}

fn query(
    invitation_id: String,
    password: String,
    pool: web::Data<Pool>,
) -> Result<SlimUser, ServiceError> {
    use crate::schema::invitations::dsl::{id, invitations};
    use crate::schema::users::dsl::users;

    let invitation_id = uuid::Uuid::parse_str(&invitation_id)?;

    let conn = &pool.get().unwrap();

    invitations
        .filter(id.eq(invitation_id))
        .load::<Invitation>(conn)
        .map_err(|_| ServiceError::BadRequest("Invalid invitation".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    let password = hash_password(&password)?;
                    dbg!(&password);

                    let user = User::from_details(invitation.email, password);
                    let inserted_user: User =
                        diesel::insert_into(users).values(&user).get_result(conn)?;
                    dbg!(&inserted_user);

                    return Ok(inserted_user.into());
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}
