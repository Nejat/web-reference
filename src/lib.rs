#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/web-reference/0.1.0")]

#![doc = include_str ! ("../readme.md")]

#[cfg(feature = "const_format")]
#[macro_use]
extern crate const_format;
#[macro_use]
extern crate lazy_regex;
#[cfg(any(any(feature = "serialize", feature = "deserialize")))]
#[macro_use]
extern crate serde;

mod models;

pub mod prelude;

/*
tag
    name
    description
    browser-support
    attributes
    events
    alternatives
tag-category
    description
    tags
attribute
    name
    description
    browser-support
    values
    alternatives
attribute-category
    description
    attributes
event
    name
    description
    category
    browser-support
    event-object
event-category
    description
    events
object
    name
    description
    fields
    methods
*/
