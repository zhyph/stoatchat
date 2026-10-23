use revolt_result::Result;

use crate::{CosmeticItem, MemberStyle, ServerStyle, UserStyle};

#[cfg(feature = "mongodb")]
mod mongodb;
mod reference;

/// Cosmetics storage. Saves replace the whole document (upsert), so removed fields really disappear.
#[async_trait]
pub trait AbstractCosmetics: Sync + Send {
    async fn fetch_user_styles(&self, ids: &[String]) -> Result<Vec<UserStyle>>;
    async fn save_user_style(&self, style: &UserStyle) -> Result<()>;

    async fn fetch_member_style(&self, server: &str, user: &str) -> Result<Option<MemberStyle>>;
    async fn save_member_style(&self, style: &MemberStyle) -> Result<()>;

    async fn fetch_server_style(&self, server: &str) -> Result<Option<ServerStyle>>;
    async fn save_server_style(&self, style: &ServerStyle) -> Result<()>;

    async fn fetch_cosmetic_items(&self) -> Result<Vec<CosmeticItem>>;
    async fn fetch_cosmetic_item(&self, id: &str) -> Result<CosmeticItem>;
    async fn insert_cosmetic_item(&self, item: &CosmeticItem) -> Result<()>;
    async fn delete_cosmetic_item(&self, id: &str) -> Result<()>;
}
