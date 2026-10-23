use revolt_database::{util::reference::Reference, Database, User};
use revolt_result::{create_error, Result};
use rocket::State;
use rocket_empty::EmptyResponse;

/// # Delete Sound
///
/// Delete a soundboard sound; the uploader or anyone who can manage customisation may do this.
#[openapi(tag = "Server Customisation")]
#[delete("/<target>/sounds/<sound_id>")]
pub async fn delete(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    sound_id: String,
) -> Result<EmptyResponse> {
    let server = target.as_server(db).await?;
    let sound = db.fetch_soundboard_sound(&sound_id).await?;
    if sound.server_id != server.id {
        return Err(create_error!(NotFound));
    }
    if sound.creator_id != user.id {
        super::require_manage(db, &user, &server).await?;
    }

    sound.delete(db).await?;
    Ok(EmptyResponse)
}
