//! Cosmetics (Nitro parity fork, see NITRO.md): fork-owned data the SDK and official images never see.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

#[cfg(feature = "validator")]
use validator::Validate;

use super::{File, RE_COLOUR};

/// Server tag: 1 to 4 uppercase letters or digits
pub static RE_SERVER_TAG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Z0-9]{1,4}$").unwrap());

/// Vanity invite code: 3 to 32 lowercase letters, digits or dashes
pub static RE_VANITY: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9-]{3,32}$").unwrap());

auto_derived!(
    /// Font for a styled display name
    #[derive(Default)]
    pub enum NameFont {
        #[default]
        Default,
        Serif,
        Mono,
        Rounded,
        Handwritten,
        Pixel,
    }

    /// Animated effect on a styled display name
    #[derive(Default)]
    pub enum NameEffect {
        #[default]
        None,
        Glow,
        Shimmer,
    }

    /// Display name style
    #[derive(Default)]
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct NameStyle {
        #[cfg_attr(feature = "serde", serde(default))]
        pub font: NameFont,
        /// Solid colour or CSS gradient
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 128), regex = "RE_COLOUR"))]
        pub colour: Option<String>,
        #[cfg_attr(feature = "serde", serde(default))]
        pub effect: NameEffect,
    }

    /// Profile card colours
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct ProfileTheme {
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 128), regex = "RE_COLOUR"))]
        pub primary: String,
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 128), regex = "RE_COLOUR"))]
        pub accent: String,
    }

    /// A user's cosmetics
    #[derive(Default)]
    pub struct UserStyle {
        #[cfg_attr(feature = "serde", serde(rename = "_id"))]
        pub id: String,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub theme: Option<ProfileTheme>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub name: Option<NameStyle>,
        /// Catalog item id (kind decoration)
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub decoration: Option<String>,
        /// Catalog item id (kind effect)
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub effect: Option<String>,
        /// Server whose tag is shown next to this user's name
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub tag_server: Option<String>,
    }

    /// Replace the caller's cosmetics (absent fields are cleared)
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct DataEditUserStyle {
        #[cfg_attr(feature = "validator", validate)]
        pub theme: Option<ProfileTheme>,
        #[cfg_attr(feature = "validator", validate)]
        pub name: Option<NameStyle>,
        pub decoration: Option<String>,
        pub effect: Option<String>,
        pub tag_server: Option<String>,
    }

    /// Look up several users' cosmetics
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct DataFetchUserStyles {
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 200)))]
        pub ids: Vec<String>,
    }

    /// Per-server profile
    #[derive(Default)]
    pub struct MemberStyle {
        pub server: String,
        pub user: String,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub banner: Option<File>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub bio: Option<String>,
    }

    /// Replace the caller's per-server profile; `banner` is an upload id (tag `backgrounds`) or omitted to remove
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct DataEditMemberStyle {
        pub banner: Option<String>,
        /// Keep the current banner instead of replacing/removing it
        #[cfg_attr(feature = "serde", serde(default))]
        pub keep_banner: bool,
        #[cfg_attr(feature = "validator", validate(length(max = 190)))]
        pub bio: Option<String>,
    }

    /// How a role's colour is drawn
    pub enum RoleStyle {
        GradientAnimated,
        Holographic,
    }

    /// A server's cosmetics
    #[derive(Default)]
    pub struct ServerStyle {
        #[cfg_attr(feature = "serde", serde(rename = "_id"))]
        pub id: String,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub tag: Option<String>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub tag_emoji: Option<String>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub invite_background: Option<File>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none", default))]
        pub vanity: Option<String>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "HashMap::is_empty", default))]
        pub role_styles: HashMap<String, RoleStyle>,
    }

    /// Edit a server's cosmetics (only the given fields change)
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct DataEditServerStyle {
        #[cfg_attr(feature = "validator", validate(regex = "RE_SERVER_TAG"))]
        pub tag: Option<String>,
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 128)))]
        pub tag_emoji: Option<String>,
        /// Upload id (tag `backgrounds`)
        pub invite_background: Option<String>,
        pub role_styles: Option<HashMap<String, RoleStyle>>,
        /// Fields to clear: Tag, InviteBackground
        #[cfg_attr(feature = "serde", serde(default))]
        pub remove: Vec<FieldsServerStyle>,
    }

    /// Clearable server style fields
    pub enum FieldsServerStyle {
        Tag,
        InviteBackground,
    }

    /// Claim a vanity invite code for a server channel
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct DataSetVanity {
        #[cfg_attr(feature = "validator", validate(regex = "RE_VANITY"))]
        pub code: String,
        pub channel: String,
    }

    /// Kind of catalog item
    pub enum CosmeticKind {
        Decoration,
        Effect,
    }

    /// An avatar decoration or profile effect anyone can pick
    pub struct CosmeticItem {
        #[cfg_attr(feature = "serde", serde(rename = "_id"))]
        pub id: String,
        pub kind: CosmeticKind,
        pub name: String,
        pub file: File,
    }

    /// Add a catalog item from an upload (tag `emojis`)
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct DataCreateCosmeticItem {
        pub file_id: String,
        pub kind: CosmeticKind,
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 32)))]
        pub name: String,
    }
);
