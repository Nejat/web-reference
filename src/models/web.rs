use crate::prelude::*;

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WebReference {
    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub attributes: Attributes,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub attributes_categorized: AttributesCategorized,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub event_objects: EventObjects,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub events: Events,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub events_categorized: EventsCategorized,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub supported_browsers: TagsSupport,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub tags: Tags,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub tags_categorized: TagsCategorized,
}
