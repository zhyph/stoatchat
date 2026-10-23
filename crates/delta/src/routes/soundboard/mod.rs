//! Soundboard (Nitro parity fork, see NITRO.md). Routes are mounted by `servers::routes()` and `channels::routes()`.

use revolt_database::{util::permissions::DatabasePermissionQuery, Database, Server, User};
use revolt_permissions::{calculate_server_permissions, ChannelPermission};
use revolt_result::Result;

pub mod sound_create;
pub mod sound_delete;
pub mod sound_edit;
pub mod sound_list;
pub mod sound_play;

/// Managing a server's sounds needs the same permission as managing its emoji
async fn require_manage(db: &Database, user: &User, server: &Server) -> Result<()> {
    let mut query = DatabasePermissionQuery::new(db, user).server(server);
    calculate_server_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::ManageCustomisation)
}
