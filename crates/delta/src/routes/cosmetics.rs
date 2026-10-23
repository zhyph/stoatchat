//! Cosmetics (Nitro parity fork, see NITRO.md): profile themes, name styles, decorations, per-server profiles,
//! server tags, invite backgrounds, vanity invites and role styles. All of it lives in fork-owned collections.

use revolt_database::{
    events::client::EventV1,
    util::{permissions::perms, reference::Reference},
    CosmeticItem, Database, File, Invite, MemberCompositeKey, MemberStyle, Server, ServerStyle,
    User, UserStyle,
};
use revolt_models::v0;
use revolt_permissions::{calculate_server_permissions, ChannelPermission};
use revolt_result::{create_error, Result};
use revolt_rocket_okapi::revolt_okapi::openapi3::OpenApi;
use rocket::{serde::json::Json, Route, State};
use rocket_empty::EmptyResponse;
use validator::Validate;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        fetch_users,
        edit_me,
        fetch_server,
        edit_server,
        set_vanity,
        fetch_member,
        edit_member,
        list_items,
        create_item,
        delete_item,
    ]
}

fn validate<T: Validate>(data: &T) -> Result<()> {
    data.validate().map_err(|error| {
        create_error!(FailedValidation {
            error: error.to_string()
        })
    })
}

async fn require(db: &Database, user: &User, server: &Server, permission: ChannelPermission) -> Result<()> {
    let mut query = perms(db, user).server(server);
    calculate_server_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(permission)
}

/// Built-in designs ship with the client as `builtin:<slug>`; anything else must be a catalog item of the right kind
async fn check_item(db: &Database, id: &Option<String>, kind: v0::CosmeticKind) -> Result<()> {
    let Some(id) = id else { return Ok(()) };
    if let Some(slug) = id.strip_prefix("builtin:") {
        if !slug.is_empty() && slug.len() <= 32 && slug.bytes().all(|b| b.is_ascii_lowercase() || b == b'-') {
            return Ok(());
        }
        return Err(create_error!(InvalidOperation));
    }
    if db.fetch_cosmetic_item(id).await?.kind != kind {
        return Err(create_error!(InvalidOperation));
    }
    Ok(())
}

/// # Fetch User Cosmetics
///
/// Cosmetics of up to 200 users; users without any are left out.
#[openapi(tag = "Cosmetics")]
#[post("/users", data = "<data>")]
async fn fetch_users(
    db: &State<Database>,
    _user: User,
    data: Json<v0::DataFetchUserStyles>,
) -> Result<Json<Vec<v0::UserStyle>>> {
    validate(&data.0)?;
    Ok(Json(
        db.fetch_user_styles(&data.0.ids)
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    ))
}

/// # Edit My Cosmetics
///
/// Replace your cosmetics; omitted fields are cleared.
#[openapi(tag = "Cosmetics")]
#[put("/users/@me", data = "<data>")]
async fn edit_me(
    db: &State<Database>,
    user: User,
    data: Json<v0::DataEditUserStyle>,
) -> Result<Json<v0::UserStyle>> {
    let data = data.into_inner();
    validate(&data)?;
    check_item(db, &data.decoration, v0::CosmeticKind::Decoration).await?;
    check_item(db, &data.effect, v0::CosmeticKind::Effect).await?;

    // Only a tag you could see: a server you're in that has one
    if let Some(server) = &data.tag_server {
        db.fetch_member(server, &user.id).await?;
        if db.fetch_server_style(server).await?.and_then(|s| s.tag).is_none() {
            return Err(create_error!(InvalidOperation));
        }
    }

    let style = UserStyle {
        id: user.id.clone(),
        theme: data.theme,
        name: data.name,
        decoration: data.decoration,
        effect: data.effect,
        tag_server: data.tag_server,
    };
    db.save_user_style(&style).await?;

    // p_user's member fan-out uses the bots-only "<server>u" topic, so publish to each server topic,
    // which every member's socket follows, plus the user topic (friends, DMs)
    let event = EventV1::CosmeticsUpdate {
        kind: "user".to_string(),
        id: user.id.clone(),
    };
    for member in db.fetch_all_memberships(&user.id).await.unwrap_or_default() {
        event.clone().p(member.id.server).await;
    }
    event.p(user.id).await;

    Ok(Json(style.into()))
}

/// # Fetch Server Cosmetics
///
/// Public, so invite pages can show the tag and background.
#[openapi(tag = "Cosmetics")]
#[get("/servers/<target>")]
async fn fetch_server(db: &State<Database>, target: Reference<'_>) -> Result<Json<v0::ServerStyle>> {
    let server = target.as_server(db).await?;
    Ok(Json(
        db.fetch_server_style(&server.id)
            .await?
            .unwrap_or(ServerStyle {
                id: server.id,
                ..Default::default()
            })
            .into(),
    ))
}

/// # Edit Server Cosmetics
///
/// Tag, tag emoji and invite background need Manage Server; role styles need Manage Roles.
#[openapi(tag = "Cosmetics")]
#[patch("/servers/<target>", data = "<data>")]
async fn edit_server(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    data: Json<v0::DataEditServerStyle>,
) -> Result<Json<v0::ServerStyle>> {
    let data = data.into_inner();
    validate(&data)?;
    let server = target.as_server(db).await?;

    let general = data.tag.is_some()
        || data.tag_emoji.is_some()
        || data.invite_background.is_some()
        || !data.remove.is_empty();
    if general {
        require(db, &user, &server, ChannelPermission::ManageServer).await?;
    }
    if let Some(styles) = &data.role_styles {
        require(db, &user, &server, ChannelPermission::ManageRole).await?;
        if styles.keys().any(|role| !server.roles.contains_key(role)) {
            return Err(create_error!(InvalidRole));
        }
    }

    let mut style = db.fetch_server_style(&server.id).await?.unwrap_or(ServerStyle {
        id: server.id.clone(),
        ..Default::default()
    });

    for field in data.remove {
        match field {
            v0::FieldsServerStyle::Tag => {
                style.tag = None;
                style.tag_emoji = None;
            }
            v0::FieldsServerStyle::InviteBackground => {
                if let Some(file) = style.invite_background.take() {
                    db.mark_attachment_as_deleted(&file.id).await.ok();
                }
            }
        }
    }
    if let Some(tag) = data.tag {
        style.tag = Some(tag);
    }
    if let Some(emoji) = data.tag_emoji {
        style.tag_emoji = Some(emoji);
    }
    if let Some(id) = data.invite_background {
        let file = File::use_background(db, &id, &server.id, &user.id).await?;
        if let Some(old) = style.invite_background.replace(file) {
            db.mark_attachment_as_deleted(&old.id).await.ok();
        }
    }
    if let Some(styles) = data.role_styles {
        style.role_styles = styles;
    }
    db.save_server_style(&style).await?;

    EventV1::CosmeticsUpdate {
        kind: "server".to_string(),
        id: server.id.clone(),
    }
    .p(server.id)
    .await;

    Ok(Json(style.into()))
}

/// # Set Vanity Invite
///
/// Claims a custom invite code (replacing this server's previous one). It is an ordinary invite,
/// so any client can join with it.
#[openapi(tag = "Cosmetics")]
#[put("/servers/<target>/vanity", data = "<data>")]
async fn set_vanity(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    data: Json<v0::DataSetVanity>,
) -> Result<Json<v0::ServerStyle>> {
    let data = data.into_inner();
    validate(&data)?;
    let server = target.as_server(db).await?;
    require(db, &user, &server, ChannelPermission::ManageServer).await?;

    let channel = db.fetch_channel(&data.channel).await?;
    if channel.server() != Some(server.id.as_str()) {
        return Err(create_error!(InvalidOperation));
    }

    let mut style = db.fetch_server_style(&server.id).await?.unwrap_or(ServerStyle {
        id: server.id.clone(),
        ..Default::default()
    });

    // Re-pointing our own code at another channel is fine; taking any other existing invite code is not
    if style.vanity.as_deref() != Some(data.code.as_str()) && db.fetch_invite(&data.code).await.is_ok() {
        return Err(create_error!(VanityTaken));
    }
    if let Some(old) = style.vanity.take() {
        db.delete_invite(&old).await.ok();
    }

    db.insert_invite(&Invite::Server {
        code: data.code.clone(),
        server: server.id.clone(),
        creator: user.id,
        channel: data.channel,
        max_uses: None,
        uses: 0,
        expires: None,
    })
    .await?;
    style.vanity = Some(data.code);
    db.save_server_style(&style).await?;

    Ok(Json(style.into()))
}

/// # Fetch Member Profile
///
/// A member's per-server banner and bio; you must be in the server.
#[openapi(tag = "Cosmetics")]
#[get("/servers/<target>/members/<member>")]
async fn fetch_member(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    member: String,
) -> Result<Json<v0::MemberStyle>> {
    let server = target.as_server(db).await?;
    db.fetch_member(&server.id, &user.id).await?;
    Ok(Json(
        db.fetch_member_style(&server.id, &member)
            .await?
            .unwrap_or(MemberStyle {
                id: MemberCompositeKey {
                    server: server.id,
                    user: member,
                },
                banner: None,
                bio: None,
            })
            .into(),
    ))
}

/// # Edit My Member Profile
///
/// Set your banner (upload tag `backgrounds`) and bio for this server.
#[openapi(tag = "Cosmetics")]
#[put("/servers/<target>/members/@me", data = "<data>")]
async fn edit_member(
    db: &State<Database>,
    user: User,
    target: Reference<'_>,
    data: Json<v0::DataEditMemberStyle>,
) -> Result<Json<v0::MemberStyle>> {
    let data = data.into_inner();
    validate(&data)?;
    let server = target.as_server(db).await?;
    db.fetch_member(&server.id, &user.id).await?;

    let mut style = db.fetch_member_style(&server.id, &user.id).await?.unwrap_or(MemberStyle {
        id: MemberCompositeKey {
            server: server.id.clone(),
            user: user.id.clone(),
        },
        banner: None,
        bio: None,
    });

    if !data.keep_banner {
        let banner = match data.banner {
            Some(id) => Some(File::use_background(db, &id, &user.id, &user.id).await?),
            None => None,
        };
        if let Some(old) = std::mem::replace(&mut style.banner, banner) {
            db.mark_attachment_as_deleted(&old.id).await.ok();
        }
    }
    style.bio = data.bio.filter(|bio| !bio.trim().is_empty());
    db.save_member_style(&style).await?;

    EventV1::CosmeticsUpdate {
        kind: "member".to_string(),
        id: format!("{}:{}", server.id, user.id),
    }
    .p(server.id)
    .await;

    Ok(Json(style.into()))
}

/// # List Cosmetic Catalog
///
/// Uploaded decorations and effects (built-in designs ship with the client).
#[openapi(tag = "Cosmetics")]
#[get("/items")]
async fn list_items(db: &State<Database>, _user: User) -> Result<Json<Vec<v0::CosmeticItem>>> {
    Ok(Json(
        db.fetch_cosmetic_items()
            .await?
            .into_iter()
            .map(Into::into)
            .collect(),
    ))
}

/// # Add Cosmetic Item
///
/// Instance admins (privileged users) add a decoration or effect from an upload (tag `emojis`).
#[openapi(tag = "Cosmetics")]
#[post("/items", data = "<data>")]
async fn create_item(
    db: &State<Database>,
    user: User,
    data: Json<v0::DataCreateCosmeticItem>,
) -> Result<Json<v0::CosmeticItem>> {
    if !user.privileged {
        return Err(create_error!(NotPrivileged));
    }
    let data = data.into_inner();
    validate(&data)?;

    // the item id is the upload id, as with emoji
    let file = File::use_emoji(db, &data.file_id, &data.file_id, &user.id).await?;
    let item = CosmeticItem {
        id: data.file_id,
        kind: data.kind,
        name: data.name,
        file,
    };
    db.insert_cosmetic_item(&item).await?;
    Ok(Json(item.into()))
}

/// # Delete Cosmetic Item
#[openapi(tag = "Cosmetics")]
#[delete("/items/<id>")]
async fn delete_item(db: &State<Database>, user: User, id: String) -> Result<EmptyResponse> {
    if !user.privileged {
        return Err(create_error!(NotPrivileged));
    }
    db.delete_cosmetic_item(&id).await.map(|_| EmptyResponse)
}
