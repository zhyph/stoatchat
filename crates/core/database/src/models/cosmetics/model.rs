//! Cosmetics (Nitro parity fork, see NITRO.md): stored in fork-owned collections only.

use std::collections::HashMap;

use revolt_models::v0;

use crate::{File, MemberCompositeKey};

auto_derived!(
    /// A user's cosmetics (`user_styles`)
    #[derive(Default)]
    pub struct UserStyle {
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub theme: Option<v0::ProfileTheme>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub name: Option<v0::NameStyle>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub decoration: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub effect: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub tag_server: Option<String>,
    }

    /// A member's per-server profile (`member_styles`)
    pub struct MemberStyle {
        #[serde(rename = "_id")]
        pub id: MemberCompositeKey,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub banner: Option<File>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub bio: Option<String>,
    }

    /// A server's cosmetics (`server_styles`)
    #[derive(Default)]
    pub struct ServerStyle {
        #[serde(rename = "_id")]
        pub id: String,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub tag: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub tag_emoji: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub invite_background: Option<File>,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        pub vanity: Option<String>,
        #[serde(skip_serializing_if = "HashMap::is_empty", default)]
        pub role_styles: HashMap<String, v0::RoleStyle>,
    }

    /// Catalog item (`cosmetic_items`)
    pub struct CosmeticItem {
        #[serde(rename = "_id")]
        pub id: String,
        pub kind: v0::CosmeticKind,
        pub name: String,
        pub file: File,
    }
);

impl From<UserStyle> for v0::UserStyle {
    fn from(value: UserStyle) -> Self {
        v0::UserStyle {
            id: value.id,
            theme: value.theme,
            name: value.name,
            decoration: value.decoration,
            effect: value.effect,
            tag_server: value.tag_server,
        }
    }
}

impl From<MemberStyle> for v0::MemberStyle {
    fn from(value: MemberStyle) -> Self {
        v0::MemberStyle {
            server: value.id.server,
            user: value.id.user,
            banner: value.banner.map(Into::into),
            bio: value.bio,
        }
    }
}

impl From<ServerStyle> for v0::ServerStyle {
    fn from(value: ServerStyle) -> Self {
        v0::ServerStyle {
            id: value.id,
            tag: value.tag,
            tag_emoji: value.tag_emoji,
            invite_background: value.invite_background.map(Into::into),
            vanity: value.vanity,
            role_styles: value.role_styles,
        }
    }
}

impl From<CosmeticItem> for v0::CosmeticItem {
    fn from(value: CosmeticItem) -> Self {
        v0::CosmeticItem {
            id: value.id,
            kind: value.kind,
            name: value.name,
            file: value.file.into(),
        }
    }
}
