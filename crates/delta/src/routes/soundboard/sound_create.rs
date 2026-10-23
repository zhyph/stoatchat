use revolt_config::config;
use revolt_database::{util::reference::Reference, Database, File, SoundboardSound, User};
use revolt_models::v0;
use revolt_result::{create_error, Result};
use rocket::{serde::json::Json, State};
use validator::Validate;

/// # Create Sound
///
/// Create a soundboard sound from a file uploaded to the `sounds` tag.
#[openapi(tag = "Server Customisation")]
#[post("/<target>/sounds", data = "<data>")]
pub async fn create(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    data: Json<v0::DataCreateSoundboardSound>,
) -> Result<Json<v0::SoundboardSound>> {
    let data = data.into_inner();
    data.validate().map_err(|error| {
        create_error!(FailedValidation {
            error: error.to_string()
        })
    })?;

    let server = target.as_server(db).await?;
    super::require_manage(db, &user, &server).await?;

    let max = config().await.features.limits.global.server_sounds;
    if db.fetch_soundboard_sounds(&server.id).await?.len() >= max {
        return Err(create_error!(TooManySoundboardSounds { max }));
    }

    // Claims the upload (tag `sounds`, audio only); its id becomes the sound id
    File::use_sound(db, &data.file_id, &user.id).await?;

    let sound = SoundboardSound {
        id: data.file_id,
        server_id: server.id,
        creator_id: user.id,
        name: data.name,
        volume: data.volume.unwrap_or(100),
        emoji: data.emoji,
    };
    sound.create(db).await?;

    Ok(Json(sound.into()))
}
