use crate::prelude::*;

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Event {
    ///
    pub name: String,

    ///
    pub belongs_to: EventBelongsTo,

    ///
    #[cfg_attr(all(feature = "serialize"), serde(skip_serializing_if = "Description::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub description: Description,

    ///
    pub details: EventDetails,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub event_objects: HasEventObjects,

    ///
    #[cfg_attr(all(feature = "serialize"), serde(skip_serializing_if = "SupportedBrowsers::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub supported: SupportedBrowsers,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub deprecated: AlternativeEvents,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EventDetails {
    ///
    pub bubbles: bool,

    ///
    pub cancelable: bool,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EventCategoryDetails {
    ///
    pub category: EventCategory,

    ///
    #[cfg_attr(all(feature = "serialize"), serde(skip_serializing_if = "Description::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub description: Description,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub events: HasEvents,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum EventCategory {
    ///
    WindowEvents,

    ///
    FormEvents,

    ///
    KeyboardEvents,

    ///
    MouseEvents,

    ///
    DragEvents,

    ///
    ClipboardEvents,

    ///
    MediaEvents,

    ///
    MiscellaneousEvents,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EventBelongsTo {
    ///
    NotDefined,

    ///
    AllVisible,

    ///
    Tags(
        #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
        #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
        #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
        #[cfg_attr(feature = "deserialize", serde(default))]
        BelongsToTags
    ),
}

impl EventBelongsTo {
    ///
    #[inline]
    #[must_use]
    pub fn has_tags(&self) -> bool {
        *self != Self::AllVisible
    }
}
