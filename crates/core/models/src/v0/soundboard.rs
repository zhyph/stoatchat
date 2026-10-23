#[cfg(feature = "validator")]
use validator::Validate;

auto_derived!(
    /// Soundboard sound
    pub struct SoundboardSound {
        /// Unique Id (same as the uploaded file's id)
        #[cfg_attr(feature = "serde", serde(rename = "_id"))]
        pub id: String,
        /// Server this sound belongs to
        pub server_id: String,
        /// Uploader user id
        pub creator_id: String,
        /// Sound name
        pub name: String,
        /// Playback volume in percent, 0 to 100
        pub volume: u8,
        /// Emoji shown next to the sound (unicode emoji or custom emoji id)
        #[cfg_attr(
            feature = "serde",
            serde(skip_serializing_if = "Option::is_none", default)
        )]
        pub emoji: Option<String>,
    }

    /// Create a soundboard sound from an uploaded file (tag `sounds`)
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct DataCreateSoundboardSound {
        /// Id of the file uploaded to the `sounds` tag
        pub file_id: String,
        /// Sound name
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 32)))]
        pub name: String,
        /// Playback volume in percent, 0 to 100
        #[cfg_attr(feature = "validator", validate(range(min = 0, max = 100)))]
        pub volume: Option<u8>,
        /// Emoji shown next to the sound
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 128)))]
        pub emoji: Option<String>,
    }

    /// Edit a soundboard sound
    #[cfg_attr(feature = "validator", derive(Validate))]
    pub struct DataEditSoundboardSound {
        /// Sound name
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 32)))]
        pub name: Option<String>,
        /// Playback volume in percent, 0 to 100
        #[cfg_attr(feature = "validator", validate(range(min = 0, max = 100)))]
        pub volume: Option<u8>,
        /// Emoji shown next to the sound
        #[cfg_attr(feature = "validator", validate(length(min = 1, max = 128)))]
        pub emoji: Option<String>,
    }

    /// Partial soundboard sound representation
    #[derive(Default)]
    pub struct PartialSoundboardSound {
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub name: Option<String>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub volume: Option<u8>,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        pub emoji: Option<String>,
    }
);
