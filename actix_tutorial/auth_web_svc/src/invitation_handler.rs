use crate::email_service::send_invitation;
use crate::errors::ServiceError;
use crate::models::{Invitation, Pool};
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::{prelude::*, PgConnection};
use futures::Future;

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    web::block(move || create_invitation(invitation_data.into_inner().email, pool)).then(|res| {
        match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServiceError),
            },
        }
    })
}

fn create_invitation(
    email: String,
    pool: web::Data<Pool>,
) -> Result<(), crate::errors::ServiceError> {
    let invitation = dbg!(query(email, pool)?);
    send_invitation(&invitation); // If you had an email service linked
    Ok(())
}

fn query(email: String, pool: web::Data<Pool>) -> Result<Invitation, crate::errors::ServiceError> {
    use crate::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = email.into();
    let conn = &pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}
