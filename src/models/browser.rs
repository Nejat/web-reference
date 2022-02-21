use std::str::FromStr;

use crate::prelude::*;

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Browser {
    ///
    Chrome,

    ///
    Edge,

    ///
    Firefox,

    ///
    Safari,

    ///
    Opera,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Support {
    ///
    Yes,

    ///
    No,

    ///
    Version {
        ///
        version: String,

        ///
        caveat: usize,
    },
}

impl FromStr for Support {
    type Err = String;

    fn from_str(src: &str) -> std::result::Result<Self, Self::Err> {
        Ok(
            match src {
                "Yes" => Self::Yes,
                "No" => Self::No,
                version if version.ends_with('*') => {
                    let number = version.trim_end_matches('*');
                    if number.parse::<f32>().is_err() {
                        return Err(format!("{version} is not valid value for version"));
                    }
                    Self::Version {
                        version: number.to_string(),
                        caveat: version.len() - number.len(),
                    }
                }
                version => {
                    Self::Version {
                        version: version.to_string(),
                        caveat: 0,
                    }
                }
            }
        )
    }
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SupportedBrowsers {
    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub supported: BrowsersSupported,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub caveats: Caveats,
}

impl SupportedBrowsers {
    ///
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.supported.is_empty() && self.caveats.is_empty()
    }
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TagsSupportedBrowsers {
    ///
    pub tag: String,

    ///
    pub browsers: SupportedBrowsers,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub attributes: AttributesSupport,
}