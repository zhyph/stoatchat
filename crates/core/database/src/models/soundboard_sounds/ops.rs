use revolt_result::Result;

use crate::{PartialSoundboardSound, SoundboardSound};

#[cfg(feature = "mongodb")]
mod mongodb;
mod reference;

#[async_trait]
pub trait AbstractSoundboardSounds: Sync + Send {
    /// Insert a sound
    async fn insert_soundboard_sound(&self, sound: &SoundboardSound) -> Result<()>;

    /// Fetch a sound by its id
    async fn fetch_soundboard_sound(&self, id: &str) -> Result<SoundboardSound>;

    /// Fetch all sounds of a server
    async fn fetch_soundboard_sounds(&self, server_id: &str) -> Result<Vec<SoundboardSound>>;

    /// Update a sound
    async fn update_soundboard_sound(
        &self,
        id: &str,
        partial: &PartialSoundboardSound,
    ) -> Result<()>;

    /// Delete a sound and mark its file deleted. The file's `used_for` is removed too: official
    /// (unforked) images can't deserialize `SoundboardSound` usage, and crond reads deleted files.
    async fn delete_soundboard_sound(&self, id: &str) -> Result<()>;
}
