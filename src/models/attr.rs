use crate::prelude::*;

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Attribute {
    ///
    pub name: String,

    ///
    pub belongs_to: AttributeBelongsTo,

    ///
    #[cfg_attr(all(feature = "serialize"), serde(skip_serializing_if = "Description::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub description: Description,

    ///
    #[cfg_attr(all(feature = "serialize"), serde(skip_serializing_if = "SupportedBrowsers::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub supported: SupportedBrowsers,

    ///
    pub values: AttributeValue,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttributeCategoryDetails {
    ///
    pub category: AttributeCategory,

    ///
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
    #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
    #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
    #[cfg_attr(feature = "deserialize", serde(default))]
    pub attributes: HasAttributes,
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttributeValue {
    ///
    None,

    ///
    Boolean {
        ///
        description: Description
    },

    ///
    BooleanAuto {
        ///
        description: Description
    },

    ///
    CharacterSet {
        ///
        description: Description
    },

    ///
    DateTime {
        ///
        description: Description
    },

    ///
    Filename {
        ///
        description: Description
    },

    ///
    HTMLCode {
        ///
        description: Description
    },

    ///
    LanguageCode {
        ///
        description: Description
    },

    ///
    Id {
        ///
        description: Description
    },

    ///
    MapName {
        ///
        description: Description
    },

    ///
    MediaType {
        ///
        description: Description
    },

    ///
    MediaQuery {
        ///
        description: Description
    },

    ///
    Number {
        ///
        description: Description
    },

    ///
    OnOff {
        ///
        description: Description
    },

    ///
    Pixels {
        ///
        description: Description
    },

    ///
    RegExp {
        ///
        description: Description
    },

    ///
    Style {
        ///
        description: Description
    },

    ///
    Text {
        ///
        description: Description
    },

    ///
    URL {
        ///
        description: Description
    },

    ///
    URLList {
        ///
        description: Description
    },

    ///
    Values(
        #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexMap::is_empty"))]
        #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
        #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashMap::is_empty"))]
        #[cfg_attr(feature = "deserialize", serde(default))]
        AttributeValues
    ),

    ///
    YesNo {
        ///
        description: Description
    },
}

impl TryFrom<AttributeValues> for AttributeValue {
    type Error = String;

    fn try_from(source: AttributeValues) -> Result<Self, Self::Error> {
        Ok(if source.is_empty() {
            Self::None
        } else if source.len() > 2 && source.values().all(|v| v.name != "auto" ){
            Self::Values(source)
        } else {
            let attr_values_set = source.keys().cloned().collect::<Vec<_>>().join(", ");
            let multiple = source.len() > 1;

            let description = source.values()
                .fold(
                    Description::default(),
                    |acc, next| if multiple {
                        acc.combine(&Description::from(format!("{}: {}", next.name, next.description)))
                    } else {
                        acc.combine(&next.description)
                    }
                );

            match attr_values_set.as_str() {
                "" | "novalidate" | "autofocus" | "checked" |
                "disabled" | "formnovalidate" | "multiple" | "readonly" |
                "required" | "selected" | "allow" | "ismap" |
                "autoplay" | "controls" | "loop" | "muted" |
                "default" | "reversed" | "open" | "async" |
                "defer" | "hidden" | "truefalse" | "TrueFalse" =>
                    Self::Boolean { description },
                "number" =>
                    Self::Number { description },
                "pixels" =>
                    Self::Pixels { description },
                "URL" =>
                    Self::URL { description },
                "URL-list" =>
                    Self::URLList { description },
                "YYYY-MM-DDThh:mm:ssTZD" | "YYYY-MM-DDThh:mm:ssTZDorPTDHMS" =>
                    Self::DateTime { description },
                "character_set" =>
                    Self::CharacterSet { description },
                "on, off" =>
                    Self::OnOff { description },
                "text" | "string" | "character" | "classname" =>
                    Self::Text { description },
                "id" | "element_id" | "datalist_id" | "header_id" =>
                    Self::Id { description },
                "filename" =>
                    Self::Filename { description },
                "rexecp" =>
                    Self::RegExp { description },
                "HTML_code" =>
                    Self::HTMLCode { description },
                "mapname" => Self::MapName { description },
                "language_code" =>
                    Self::LanguageCode { description },
                "media_type" =>
                    Self::MediaType { description },
                "media_query" =>
                    Self::MediaQuery { description },
                "style_definitions" =>
                    Self::Style { description },
                "true, false, auto" =>
                    Self::BooleanAuto { description },
                "yes, no" =>
                    Self::YesNo { description },
                _ => Self::Values(source)
            }
        })
    }
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
// todo: untagged generates null values for simple named variants
// #[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case", untagged))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AttributeBelongsTo {
    ///
    Tags(
        #[cfg_attr(all(feature = "serialize", feature = "ordered-map"), serde(skip_serializing_if = "indexmap::IndexSet::is_empty"))]
        #[cfg_attr(all(feature = "serialize", feature = "ordered-map", feature = "serde_json"), serde(with = "indexmap::serde_seq"))]
        #[cfg_attr(all(feature = "serialize", not(feature = "ordered-map")), serde(skip_serializing_if = "HashSet::is_empty"))]
        #[cfg_attr(feature = "deserialize", serde(default))]
        BelongsToTags
    ),

    ///
    Global,

    ///
    NotSupported,
}

impl AttributeBelongsTo {
    ///
    #[inline]
    #[must_use]
    pub fn has_tags(&self) -> bool {
        !(*self == Self::Global || *self == Self::NotSupported)
    }
}

///
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(rename_all = "kebab-case"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AttributeCategory {
    ///
    GlobalAttributes,

    ///
    SpecificToTags,

    ///
    NotSupported,
}

impl From<AttributeBelongsTo> for AttributeCategory {
    #[inline]
    fn from(value: AttributeBelongsTo) -> Self {
        Self::from(&value)
    }
}

impl<'a> From<&'a AttributeBelongsTo> for AttributeCategory {
    fn from(value: &'a AttributeBelongsTo) -> Self {
        match value {
            AttributeBelongsTo::Tags(_) => Self::SpecificToTags,
            AttributeBelongsTo::Global => Self::GlobalAttributes,
            AttributeBelongsTo::NotSupported => Self::NotSupported
        }
    }
}
