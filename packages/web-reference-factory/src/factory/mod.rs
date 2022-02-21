use std::fs::File;
use std::io::BufWriter;

use anyhow::Result;

use web_reference::prelude::*;

use crate::common::*;

mod build;
mod scrape;

pub fn run_factory() -> Result<()> {
    let (tags, tags_categorized, tag_details) = {
        let tags_by_category = scrape_tags_by_category_page()?;
        let tag_details = scrape::scrape_tag_detail_pages(&tags_by_category)?;

        let mut tags = build::build_tags(&tags_by_category, &tag_details)?;
        let mut tags_categorized = build::categorize_tags(&tags_by_category)?;

        tags.sort_keys();
        tags_categorized.sort_keys();

        (tags, tags_categorized, tag_details)
    };

    let (attributes, attributes_categorized, events_attrs) = {
        let (attributes, events) = scrape::scrape_attributes_page()?;
        let global_attributes = scrape_global_attributes_page()?;

        let mut attributes_categorized = build::categorize_attributes(&attributes)?;
        let mut attributes = build::build_attributes(
            attributes, global_attributes, tag_details, &attributes_categorized
        )?;

        attributes_categorized.sort_keys();
        attributes.sort_keys();

        (attributes, attributes_categorized, events)
    };

    let (events, events_categorized, event_objects) = {
        let events = scrape::scrape_events_page()?;
        let events_by_category = scrape::scrape_events_by_category_page()?;
        let event_objects = scrape::scrape_event_objects_page()?;

        let mut events = build::build_events(events, events_attrs)?;
        let mut events_categorized = build::categorize_events(events_by_category)?;
        let mut event_objects = build::event_objects(event_objects)?;

        events.sort_keys();
        events_categorized.sort_keys();
        event_objects.sort_keys();

        (events, events_categorized, event_objects)
    };

    let supported_browsers = {
        let supported = scrape::scrape_browser_support_page()?;

        let mut supported_browsers = build::build_supported_browsers(supported);

        supported_browsers.sort_keys();

        supported_browsers
    };

    let reference = WebReference {
        attributes,
        attributes_categorized,
        event_objects,
        events,
        events_categorized,
        supported_browsers,
        tags,
        tags_categorized,
    };

    let output = File::create("html-5-specs.json")?;
    let writer = BufWriter::new(output);

    serde_json::to_writer_pretty(writer, &reference)?;

    Ok(())
}
