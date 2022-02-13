#![allow(clippy::wildcard_imports)]

//!

#[cfg(not(feature = "ordered-map"))]
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};

#[cfg(feature = "ordered-map")]
use indexmap::{IndexMap, IndexSet};

use super::attr::*;
use super::browser::*;
use super::common::*;
use super::event::*;
use super::tag::*;

///
#[cfg(not(feature = "ordered-map"))]
pub type Map<K, V> = HashMap<K, V>;

///
#[cfg(not(feature = "ordered-map"))]
pub type Set<V> = HashSet<V>;

///
#[cfg(feature = "ordered-map")]
pub type Map<K, V> = IndexMap<K, V>;

///
#[cfg(feature = "ordered-map")]
pub type Set<V> = IndexSet<V>;

///
pub type AlternativeEvents = Set<String>;

///
pub type AlternativeTags = Set<String>;

///
pub type AlternativeAttributes = Set<String>;

///
pub type TagAttributes = Map<String, Attribute>;

///
pub type Attributes = Map<String, TagAttributes>;

///
pub type AttributeValues = Map<String, Value>;

///
pub type AttributesCategorized = Map<AttributeCategory, AttributeCategoryDetails>;

///
pub type AttributesSupport = Map<String, BrowsersSupported>;

///
pub type BelongsToTags = Set<String>;

///
pub type BrowsersSupported = Map<Browser, Support>;

///
pub type CategorizedTags = Map<TagCategoryDetails, Tags>;

///
pub type Caveats = Set<Description>;

///
pub type EventObjects = Map<String, Object>;

///
pub type Events = Map<String, Event>;

///
pub type EventsCategorized = Map<EventCategory, EventCategoryDetails>;

///
pub type EventsSupport = Map<String, SupportedBrowsers>;

///
pub type Fields = Map<String, Field>;

///
pub type HasAttributes = Set<String>;

///
pub type HasEventObjects = Set<String>;

///
pub type HasEvents = Set<String>;

///
pub type HasTags = Set<String>;

///
pub type LanguageCodes = Set<String>;

///
pub type Methods = Map<String, Method>;

///
pub type Parameters = Map<String, Parameter>;

///
pub type Supported = Map<String, BrowsersSupported>;

///
pub type Tags = Map<String, Tag>;

///
pub type TagsCategorized = Map<TagCategory, TagCategoryDetails>;

///
pub type TagsSupport = Map<String, TagsSupportedBrowsers>;


///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct Value {
    ///
    pub name: String,

    ///
    #[cfg_attr(all(feature = "serialize"), serde(skip_serializing_if = "Description::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub description: Description,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Description(String);

impl AsRef<str> for Description {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Description {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str(self.as_ref())
    }
}

impl From<&str> for Description {
    fn from(src: &str) -> Self {
        // normalize chopped up text

        let description = src.replace(' ', "")
            .split('\n')
            .map(str::trim)
            .collect::<Vec<_>>()
            .join(" ")
            .split('.')
            .map(str::trim)
            .collect::<Vec<_>>()
            .join(".");

        let description = regex_replace_all!(r#"\.([A-Z])"#, &description, |_, begin| format!(". {begin}"));
        let description = regex_replace_all!(r#"\s+"#, &description, |_| " ");

        Self(description.to_string())
    }
}

impl From<String> for Description {
    #[inline]
    fn from(src: String) -> Self {
        Self::from(&src)
    }
}

impl From<&String> for Description {
    #[inline]
    fn from(src: &String) -> Self {
        Self::from(src.as_str())
    }
}

impl Description {
    ///
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    ///
    #[must_use]
    pub fn combine(&self, other: &Self) -> Self {
        match (&self.0, &other.0) {
            (a, b) if b.is_empty() || a == b || a.contains(b) =>
                self.clone(),
            (a, b) if a.is_empty() || b.contains(a) =>
                other.clone(),
            (a, b) if a.len() < b.len() =>
                Self(format!("{a}\n\n{b}")),
            (a, b) =>
                Self(format!("{b}\n\n{a}"))
        }
    }
}
