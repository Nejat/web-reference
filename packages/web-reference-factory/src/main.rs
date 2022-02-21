#![feature(drain_filter)]

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::wildcard_imports)]
// ==============================================================

#![doc = include_str ! ("../readme.md")]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

use std::fs;

use anyhow::Result;

#[cfg(all(feature = "analyze", not(feature = "build")))]
use crate::analyze::run_factory;
#[cfg(all(feature = "build", not(feature = "analyze")))]
use crate::factory::run_factory;

#[cfg(feature = "analyze")]
mod analyze;
#[cfg(feature = "build")]
mod factory;

mod common;
mod ignored;
mod types;

const DEBUG_TEXT_FILE: &str = "debug.txt";

// retrieved html documents are cached when online
// cached files are used unless REFRESH is true
const REFRESH: bool = false;

// todo scrape language code
// todo scrape country codes
// todo scrape event objects
// todo scrape methods

fn main() -> Result<()> {
    // for debugging
    fs::remove_file(DEBUG_TEXT_FILE).map_or((), |_err| ());

    run_factory()
}

/*
tag-category
    tag
        name
        description
        browser-support
        attributes
            name
            description
            browser-support
            values
            alternatives
        events
            name
            description
            browser-support
            details; cancelable, bubbles, etc.
            object
                fields
                    name
                    description
                    type
                    values
                methods
                    name
                    description
                    parameters
                        name
                        description
                        type
                        values
lookup
    language-codes
    countries
    .
    .
    .
 */
