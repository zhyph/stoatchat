use revolt_result::Result;

use crate::ReferenceDb;
use crate::{CosmeticItem, MemberStyle, ServerStyle, UserStyle};

use super::AbstractCosmetics;

#[async_trait]
impl AbstractCosmetics for ReferenceDb {
    async fn fetch_user_styles(&self, ids: &[String]) -> Result<Vec<UserStyle>> {
        let styles = self.user_styles.lock().await;
        Ok(ids.iter().filter_map(|id| styles.get(id).cloned()).collect())
    }

    async fn save_user_style(&self, style: &UserStyle) -> Result<()> {
        self.user_styles.lock().await.insert(style.id.clone(), style.clone());
        Ok(())
    }

    async fn fetch_member_style(&self, server: &str, user: &str) -> Result<Option<MemberStyle>> {
        Ok(self
            .member_styles
            .lock()
            .await
            .get(&(server.to_string(), user.to_string()))
            .cloned())
    }

    async fn save_member_style(&self, style: &MemberStyle) -> Result<()> {
        self.member_styles.lock().await.insert(
            (style.id.server.clone(), style.id.user.clone()),
            style.clone(),
        );
        Ok(())
    }

    async fn fetch_server_style(&self, server: &str) -> Result<Option<ServerStyle>> {
        Ok(self.server_styles.lock().await.get(server).cloned())
    }

    async fn save_server_style(&self, style: &ServerStyle) -> Result<()> {
        self.server_styles.lock().await.insert(style.id.clone(), style.clone());
        Ok(())
    }

    async fn fetch_cosmetic_items(&self) -> Result<Vec<CosmeticItem>> {
        Ok(self.cosmetic_items.lock().await.values().cloned().collect())
    }

    async fn fetch_cosmetic_item(&self, id: &str) -> Result<CosmeticItem> {
        self.cosmetic_items
            .lock()
            .await
            .get(id)
            .cloned()
            .ok_or_else(|| create_error!(NotFound))
    }

    async fn insert_cosmetic_item(&self, item: &CosmeticItem) -> Result<()> {
        self.cosmetic_items.lock().await.insert(item.id.clone(), item.clone());
        Ok(())
    }

    async fn delete_cosmetic_item(&self, id: &str) -> Result<()> {
        self.cosmetic_items
            .lock()
            .await
            .remove(id)
            .map(|_| ())
            .ok_or_else(|| create_error!(NotFound))
    }
}
