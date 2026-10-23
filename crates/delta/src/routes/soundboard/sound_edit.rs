use revolt_database::{util::reference::Reference, Database, PartialSoundboardSound, User};
use revolt_models::v0;
use revolt_result::{create_error, Result};
use rocket::{serde::json::Json, State};
use validator::Validate;

/// # Edit Sound
///
/// Change a soundboard sound's name, volume or emoji.
#[openapi(tag = "Server Customisation")]
#[patch("/<target>/sounds/<sound_id>", data = "<data>")]
pub async fn edit(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    sound_id: String,
    data: Json<v0::DataEditSoundboardSound>,
) -> Result<Json<v0::SoundboardSound>> {
    let data = data.into_inner();
    data.validate().map_err(|error| {
        create_error!(FailedValidation {
            error: error.to_string()
        })
    })?;

    let server = target.as_server(db).await?;
    let mut sound = db.fetch_soundboard_sound(&sound_id).await?;
    if sound.server_id != server.id {
        return Err(create_error!(NotFound));
    }
    if sound.creator_id != user.id {
        super::require_manage(db, &user, &server).await?;
    }

    sound
        .update(
            db,
            PartialSoundboardSound {
                name: data.name,
                volume: data.volume,
                emoji: data.emoji,
                ..Default::default()
            },
        )
        .await?;

    Ok(Json(sound.into()))
}
