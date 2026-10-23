use bson::Document;
use revolt_result::Result;

use crate::MongoDb;
use crate::{PartialSoundboardSound, SoundboardSound};

use super::AbstractSoundboardSounds;

static COL: &str = "soundboard_sounds";

#[async_trait]
impl AbstractSoundboardSounds for MongoDb {
    async fn insert_soundboard_sound(&self, sound: &SoundboardSound) -> Result<()> {
        query!(self, insert_one, COL, &sound).map(|_| ())
    }

    async fn fetch_soundboard_sound(&self, id: &str) -> Result<SoundboardSound> {
        query!(self, find_one_by_id, COL, id)?.ok_or_else(|| create_error!(NotFound))
    }

    async fn fetch_soundboard_sounds(&self, server_id: &str) -> Result<Vec<SoundboardSound>> {
        query!(self, find, COL, doc! { "server_id": server_id })
    }

    async fn update_soundboard_sound(
        &self,
        id: &str,
        partial: &PartialSoundboardSound,
    ) -> Result<()> {
        query!(self, update_one_by_id, COL, id, partial, vec![], None).map(|_| ())
    }

    async fn delete_soundboard_sound(&self, id: &str) -> Result<()> {
        query!(self, delete_one_by_id, COL, id)?;

        self.col::<Document>("attachments")
            .update_one(
                doc! { "_id": id },
                doc! {
                    "$set": { "deleted": true },
                    "$unset": { "used_for": 1 }
                },
            )
            .await
            .map(|_| ())
            .map_err(|_| create_database_error!("update_one", "attachments"))
    }
}
