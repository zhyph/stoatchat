use revolt_database::{
    events::client::EventV1,
    util::{permissions::perms, reference::Reference},
    voice::{is_in_voice_channel, UserVoiceChannel},
    Database, User,
};
use revolt_permissions::{calculate_channel_permissions, ChannelPermission};
use revolt_result::{create_error, Result};
use rocket::State;
use rocket_empty::EmptyResponse;

/// # Play Sound
///
/// Play a soundboard sound in the voice channel you're connected to. The sound may come from any
/// server you're a member of; everyone in the call plays it locally.
#[openapi(tag = "Voice")]
#[post("/<target>/sounds/<sound_id>/play")]
pub async fn play(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    sound_id: String,
) -> Result<EmptyResponse> {
    let channel = target.as_channel(db).await?;
    if channel.voice().is_none() {
        return Err(create_error!(NotAVoiceChannel));
    }

    let mut query = perms(db, &user).channel(&channel);
    calculate_channel_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::Speak)?;

    if !is_in_voice_channel(&user.id, &UserVoiceChannel::from_channel(&channel)).await? {
        return Err(create_error!(NotConnected));
    }

    let sound = db.fetch_soundboard_sound(&sound_id).await?;
    // "use anywhere": any server the player belongs to, not just this one
    db.fetch_member(&sound.server_id, &user.id)
        .await
        .map_err(|_| create_error!(NotFound))?;

    EventV1::SoundboardPlay {
        channel: channel.id().to_string(),
        user: user.id,
        sound: sound.id,
        volume: sound.volume,
    }
    .p(channel.id().to_string())
    .await;

    Ok(EmptyResponse)
}
