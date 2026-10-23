use revolt_result::Result;

use crate::ReferenceDb;
use crate::{PartialSoundboardSound, SoundboardSound};

use super::AbstractSoundboardSounds;

#[async_trait]
impl AbstractSoundboardSounds for ReferenceDb {
    async fn insert_soundboard_sound(&self, sound: &SoundboardSound) -> Result<()> {
        let mut sounds = self.soundboard_sounds.lock().await;
        if sounds.contains_key(&sound.id) {
            Err(create_database_error!("insert", "soundboard_sound"))
        } else {
            sounds.insert(sound.id.clone(), sound.clone());
            Ok(())
        }
    }

    async fn fetch_soundboard_sound(&self, id: &str) -> Result<SoundboardSound> {
        let sounds = self.soundboard_sounds.lock().await;
        sounds
            .get(id)
            .cloned()
            .ok_or_else(|| create_error!(NotFound))
    }

    async fn fetch_soundboard_sounds(&self, server_id: &str) -> Result<Vec<SoundboardSound>> {
        let sounds = self.soundboard_sounds.lock().await;
        Ok(sounds
            .values()
            .filter(|sound| sound.server_id == server_id)
            .cloned()
            .collect())
    }

    async fn update_soundboard_sound(
        &self,
        id: &str,
        partial: &PartialSoundboardSound,
    ) -> Result<()> {
        let mut sounds = self.soundboard_sounds.lock().await;
        let sound = sounds.get_mut(id).ok_or_else(|| create_error!(NotFound))?;
        if let Some(name) = &partial.name {
            sound.name = name.clone();
        }
        if let Some(volume) = partial.volume {
            sound.volume = volume;
        }
        if let Some(emoji) = &partial.emoji {
            sound.emoji = Some(emoji.clone());
        }
        Ok(())
    }

    async fn delete_soundboard_sound(&self, id: &str) -> Result<()> {
        self.soundboard_sounds
            .lock()
            .await
            .remove(id)
            .ok_or_else(|| create_error!(NotFound))?;

        if let Some(file) = self.files.lock().await.get_mut(id) {
            file.deleted = Some(true);
            file.used_for = None;
        }
        Ok(())
    }
}
