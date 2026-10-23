use bson::Document;
use mongodb::options::ReplaceOptions;
use revolt_result::Result;
use serde::Serialize;

use crate::MongoDb;
use crate::{CosmeticItem, MemberStyle, ServerStyle, UserStyle};

use super::AbstractCosmetics;

static USERS: &str = "user_styles";
static MEMBERS: &str = "member_styles";
static SERVERS: &str = "server_styles";
static ITEMS: &str = "cosmetic_items";

impl MongoDb {
    /// Replace (or insert) a whole document by `_id`
    async fn replace_doc<T: Serialize + Send + Sync>(
        &self,
        collection: &'static str,
        id: bson::Bson,
        value: &T,
    ) -> Result<()> {
        self.col::<T>(collection)
            .replace_one(doc! { "_id": id }, value)
            .with_options(ReplaceOptions::builder().upsert(true).build())
            .await
            .map(|_| ())
            .map_err(|_| create_database_error!("replace_one", collection))
    }
}

#[async_trait]
impl AbstractCosmetics for MongoDb {
    async fn fetch_user_styles(&self, ids: &[String]) -> Result<Vec<UserStyle>> {
        query!(self, find, USERS, doc! { "_id": { "$in": ids } })
    }

    async fn save_user_style(&self, style: &UserStyle) -> Result<()> {
        self.replace_doc(USERS, style.id.clone().into(), style).await
    }

    async fn fetch_member_style(&self, server: &str, user: &str) -> Result<Option<MemberStyle>> {
        query!(self, find_one, MEMBERS, doc! { "_id.server": server, "_id.user": user })
    }

    async fn save_member_style(&self, style: &MemberStyle) -> Result<()> {
        let id = bson::to_bson(&style.id)
            .map_err(|_| create_database_error!("to_bson", MEMBERS))?;
        self.replace_doc(MEMBERS, id, style).await
    }

    async fn fetch_server_style(&self, server: &str) -> Result<Option<ServerStyle>> {
        query!(self, find_one_by_id, SERVERS, server)
    }

    async fn save_server_style(&self, style: &ServerStyle) -> Result<()> {
        self.replace_doc(SERVERS, style.id.clone().into(), style).await
    }

    async fn fetch_cosmetic_items(&self) -> Result<Vec<CosmeticItem>> {
        query!(self, find, ITEMS, doc! {})
    }

    async fn fetch_cosmetic_item(&self, id: &str) -> Result<CosmeticItem> {
        query!(self, find_one_by_id, ITEMS, id)?.ok_or_else(|| create_error!(NotFound))
    }

    async fn insert_cosmetic_item(&self, item: &CosmeticItem) -> Result<()> {
        query!(self, insert_one, ITEMS, &item).map(|_| ())
    }

    async fn delete_cosmetic_item(&self, id: &str) -> Result<()> {
        query!(self, delete_one_by_id, ITEMS, id)?;
        // release the file like emoji detach does: mark deleted, keep it fallback-safe (no fork-only enum values)
        self.col::<Document>("attachments")
            .update_one(doc! { "_id": id }, doc! { "$set": { "deleted": true } })
            .await
            .map(|_| ())
            .map_err(|_| create_database_error!("update_one", "attachments"))
    }
}
