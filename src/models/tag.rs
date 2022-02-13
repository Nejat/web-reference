// use std::str::FromStr;

use crate::prelude::*;

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tag {
    ///
    pub name: String,

    ///
    #[cfg_attr(all(feature = "serialize"), serde(skip_serializing_if = "Description::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub description: Description,

    ///
    #[cfg_attr(all(feature = "serialize"), serde(skip_serializing_if = "SupportedBrowsers::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub supported: SupportedBrowsers,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub attributes: HasAttributes,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub optional_attributes: HasAttributes,

    ///
    pub global_attributes: bool,

    ///
    pub global_events: bool,

    ///
    pub alternatives: TagAlternatives,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagCategoryDetails {
    ///
    pub category: TagCategory,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub tags: HasTags,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TagAlternatives {
    ///
    None,

    ///
    Css,

    ///
    Tags(
        #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
        #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
        #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
        #[cfg_attr(feature = "deserialize", serde(default))]
        AlternativeTags
    ),
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TagCategory {
    ///
    Basic,

    ///
    Formatting,

    ///
    FormsInput,

    ///
    Frames,

    ///
    Images,

    ///
    AudioVideo,

    ///
    Links,

    ///
    Lists,

    ///
    Tables,

    ///
    StylesSemantics,

    ///
    Meta,

    ///
    Programming,
}
