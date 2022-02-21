#[cfg(feature = "analyze")]
use std::collections::HashMap;

use reqwest::Url;

use web_reference::prelude::*;

#[cfg(feature = "analyze")]
pub type DebugPageSections = HashMap<String, DebugTagNames>;

#[cfg(feature = "analyze")]
pub type DebugTagNames = Vec<String>;

#[cfg(feature = "build")]
pub type RawAttributes = Vec<RawDetails>;

#[cfg(feature = "build")]
pub type RawAttributesSupport = Vec<(String, BrowsersSupported)>;

#[cfg(feature = "build")]
pub type RawBrowserSupport = Vec<(String, BrowsersSupported, RawAttributesSupport, RawCaveats)>;

#[cfg(feature = "build")]
pub type RawDetails = (String, Url, RawBelongsTo, String);

pub type RawCaveats = Vec<String>;

#[cfg(feature = "build")]
pub type RawEvent = (String, String, Option<AlternativeEvents>, Option<RawEventDetails>, RawEventObjects);

#[cfg(feature = "build")]
pub type RawEventCategory = (Option<String>, RawEventNames);

#[cfg(feature = "build")]
pub type RawEventDetails = (SupportedBrowsers, RawTechDetails);

#[cfg(feature = "build")]
pub type RawEventNames = Vec<String>;

#[cfg(feature = "build")]
pub type RawEventObjects = Map<String, Url>;

#[cfg(feature = "build")]
pub type RawEvents = Vec<RawEvent>;

#[cfg(feature = "build")]
pub type RawEventsByCategory = Vec<(String, RawEventCategory)>;

pub type RawGlobalAttributes = Vec<(String, RawGlobalAttributeDetails, String)>;

pub type RawGlobalAttributeDetails = (BrowsersSupported, AttributeValues, RawCaveats);

pub type RawTag = (Description, bool, TagAlternatives);

pub type RawTagDetails = (Option<SupportedBrowsers>, Option<TagAttributes>, Option<bool>, Option<bool>, Option<TagAttributes>);

pub type RawTags = Vec<(String, Url, RawTag)>;

pub type RawTagsByCategory = Vec<(String, RawTags)>;

#[cfg(feature = "build")]
pub type RawTagsDetails = Map<String, RawTagDetails>;

#[cfg(feature = "build")]
pub type RawTechDetails = Map<String, String>;

#[cfg(feature = "build")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RawBelongsTo {
    ///
    AllVisible,

    ///
    Tags(BelongsToTags),

    ///
    Global,

    ///
    NotSupported,
}

#[cfg(feature = "build")]
impl TryFrom<String> for RawBelongsTo {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "Global Attributes" => Self::Global,
            "Not supported in HTML 5." => Self::NotSupported,
            "All visible elements." => Self::AllVisible,
            tags if tags.starts_with('<') => Self::Tags(
                tags.split(',')
                    .map(|v|
                        v.trim()
                            .trim_start_matches('<')
                            .trim_end_matches('>')
                            .replace(' ', "")
                    ).collect()
            ),
            unexpected => bail!("Unexpected tag for attribute: {unexpected:?}")
        })
    }
}
