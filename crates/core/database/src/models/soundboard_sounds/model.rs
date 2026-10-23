use revolt_models::v0;
use revolt_result::Result;

use crate::events::client::EventV1;
use crate::Database;

auto_derived_partial!(
    /// Soundboard sound (stored in `soundboard_sounds`; the id is the `sounds` upload's file id)
    pub struct SoundboardSound {
        /// Unique Id
        #[serde(rename = "_id")]
        pub id: String,
        /// Server this sound belongs to
        pub server_id: String,
        /// Uploader user id
        pub creator_id: String,
        /// Sound name
        pub name: String,
        /// Playback volume in percent, 0 to 100
        pub volume: u8,
        /// Emoji shown next to the sound
        #[serde(skip_serializing_if = "Option::is_none")]
        pub emoji: Option<String>,
    },
    "PartialSoundboardSound"
);

impl SoundboardSound {
    /// Save a new sound and announce it to the server
    pub async fn create(&self, db: &Database) -> Result<()> {
        db.insert_soundboard_sound(self).await?;

        EventV1::SoundboardSoundCreate(self.clone().into())
            .p(self.server_id.clone())
            .await;

        Ok(())
    }

    /// Update name, volume or emoji
    pub async fn update(&mut self, db: &Database, partial: PartialSoundboardSound) -> Result<()> {
        if let Some(name) = &partial.name {
            self.name = name.clone();
        }
        if let Some(volume) = partial.volume {
            self.volume = volume;
        }
        if let Some(emoji) = &partial.emoji {
            self.emoji = Some(emoji.clone());
        }

        db.update_soundboard_sound(&self.id, &partial).await?;

        EventV1::SoundboardSoundUpdate {
            id: self.id.clone(),
            server: self.server_id.clone(),
            data: v0::PartialSoundboardSound {
                name: partial.name,
                volume: partial.volume,
                emoji: partial.emoji,
            },
        }
        .p(self.server_id.clone())
        .await;

        Ok(())
    }

    /// Delete the sound and release its file
    pub async fn delete(&self, db: &Database) -> Result<()> {
        db.delete_soundboard_sound(&self.id).await?;

        EventV1::SoundboardSoundDelete {
            id: self.id.clone(),
            server: self.server_id.clone(),
        }
        .p(self.server_id.clone())
        .await;

        Ok(())
    }
}

impl From<SoundboardSound> for v0::SoundboardSound {
    fn from(value: SoundboardSound) -> Self {
        v0::SoundboardSound {
            id: value.id,
            server_id: value.server_id,
            creator_id: value.creator_id,
            name: value.name,
            volume: value.volume,
            emoji: value.emoji,
        }
    }
}
