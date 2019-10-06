use crate::errors::ServiceError;
use crate::models::Invitation;

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
    // Logic for email service here

    let recipient = invitation.email.as_str();

    dbg!("Sending invitation to: {}", recipient);

    Ok(())
}
