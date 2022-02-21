use anyhow::Result;
use reqwest::Url;
use select::node::Node;
use select::predicate;
use select::predicate::Predicate;

use web_reference::prelude::*;

use crate::common::*;
use crate::types::*;

#[cfg(feature = "build")]
pub const EVENT_PREFIX: &str = "on";

pub fn scrape_attributes_page() -> Result<(RawAttributes, RawAttributes)> {
    const DOC_URL: &str = "https://www.w3schools.com/tags/ref_attributes.asp";
    const DOC_TOPIC: &str = "Tag Attributes";

    let root = retrieve_document(&DOC_URL, DOC_TOPIC)?;

    let attributes = root.find(predicate::Class("w3-main")).next()
        .ok_or_else(|| anyhow!("Could not find expected contents: {DOC_TOPIC} - {DOC_URL}"))?
        .find(predicate::Class("ws-table-all")).next()
        .ok_or_else(|| anyhow!("Could not find table: {DOC_TOPIC} - {DOC_URL}"))?
        .find(predicate::Name("tr")).skip(1);

    let mut attributes = attributes.map(|row| {
        let mut columns = row.find(predicate::Name("td"));

        let attribute = if let Some(attribute) = columns.next() {
            attribute
        } else {
            let html = row.html();

            bail!("Could not find attribute: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}");
        };

        let name = attribute.text();

        let belongs_to = if let Some(belongs_to) = columns.next() {
            RawBelongsTo::try_from(belongs_to.text())?
        } else {
            let html = attribute.html();

            bail!("Could not find {name:?} attribute's belong to: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}");
        };

        let url = if belongs_to == RawBelongsTo::NotSupported {
            Url::parse(BASE_TAGS_URL)?
        } else {
            let is_event = name.starts_with(EVENT_PREFIX);
            let link = attribute.find(predicate::Name("a")).next();

            match link {
                Some(v) => {
                    let address = v.attr("href").map_or_else(
                        || {
                            let html = attribute.html();

                            Err(anyhow!("Attribute {name:?} is missing a link: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}"))
                        },
                        Ok,
                    )?;

                    absolute_url(BASE_TAGS_URL, address)?
                }
                None if is_event => Url::parse(BASE_TAGS_URL)?,
                None => bail!("Could not find {name:?} link: {DOC_TOPIC} - {DOC_URL}")
            }
        };

        let description = if let Some(description) = columns.next() {
            description.text()
        } else {
            let html = attribute.html();

            bail!("Could not find {name:?} attribute's description: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}");
        };

        Ok((name, url, belongs_to, description))
    }).collect::<Result<RawAttributes>>()?;

    Ok((
        attributes.drain_filter(|(name, _, _, _)| !name.starts_with(EVENT_PREFIX)).collect(),
        attributes.drain_filter(|(name, _, _, _)| name.starts_with(EVENT_PREFIX)).collect()
    ))
}

pub fn scrape_browser_support_page() -> Result<RawBrowserSupport> {
    const DOC_URL: &str = "https://www.w3schools.com/tags/ref_html_browsersupport.asp";
    const DOC_TOPIC: &str = "Browser Support";

    let root = retrieve_document(&DOC_URL, DOC_TOPIC)?;

    let tables = root.find(predicate::Name("table").and(predicate::Class("browserref")));

    tables.map(|table| {
        let mut rows = table.find(predicate::Name("tr")).skip(1);

        let (tag, support) = parse_browser_support(
            rows.next().ok_or_else(|| anyhow!("Empty support table: {DOC_TOPIC} - {DOC_URL}"))?
        )?;

        let attributes = rows
            .map(parse_browser_support)
            .collect::<Result<RawAttributesSupport>>()?;

        let caveats = table
            .iter_sibling(predicate::Name("p"), predicate::Name("br"))
            .map(|n| n.text())
            .collect::<RawCaveats>();

        Ok((tag, support, attributes, caveats))
    }).collect::<Result<RawBrowserSupport>>()
}

pub fn scrape_events_by_category_page() -> Result<RawEventsByCategory> {
    const DOC_URL: &str = "https://www.w3schools.com/tags/ref_eventattributes.asp";
    const DOC_TOPIC: &str = "Events by Category";

    let root = retrieve_document(&DOC_URL, DOC_TOPIC)?;

    let main = root.find(predicate::Class("w3-main")).next()
        .ok_or_else(|| anyhow!("Could not find expected contents of {DOC_TOPIC} - {DOC_URL}"))?;

    // iterate all categories with table of defined events
    main.find(predicate::Name("h2"))
        .filter(|hdr| {
            let hdr_text = hdr.text();

            hdr_text != "Global Event Attributes" &&
                hdr_text.contains("Event")
        })
        .map(
            |category| {
                let category_name = category.text();
                let category_description = category.iter_sibling(predicate::Name("p"), predicate::Name("table"))
                    .map(|desc| desc.text())
                    .next();

                let tip_url = category
                    .iter_sibling(predicate::Name("div").and(predicate::Class("w3-note")), predicate::Name("table"))
                    .filter_map(|tip| {
                        tip.find(predicate::Name("a")).next()
                    })
                    .map(|tip| tip.attr("href"))
                    .next();

                if let Some(Some(tip_url)) = tip_url {
                    let root = retrieve_document(&absolute_url(BASE_TAGS_URL, tip_url)?, &format!("{category_name:?} Tip Events"))?;

                    let events = root
                        .find(predicate::Name("h2")).find(|nd| nd.text().ends_with("Events"))
                        .ok_or_else(|| anyhow!("Could not find {category_name:?} Tip Events defined - {DOC_URL}"))?;

                    match parse_tip_events(events, &category_name) {
                        Ok(tips_table) => Ok((category_name.clone(), (category_description, parse_event_category(tips_table)?))),
                        Err(err) => Err(anyhow!("Exception parsing tip events - {DOC_URL}\n  Err: {err}"))
                    }
                } else {
                    let events_table =
                        category
                            .iter_sibling(predicate::Name("table"), predicate::Name("hr"))
                            .next()
                            .ok_or_else(|| anyhow!("Could not find {category_name:?} Events defined - {DOC_URL}"))?;

                    Ok((category_name.clone(), (category_description, parse_event_category(events_table)?)))
                }
            }
        )
        .collect::<Result<RawEventsByCategory>>()
}

pub fn scrape_events_page() -> Result<RawEvents> {
    const DOC_URL: &str = "https://www.w3schools.com/jsref/dom_obj_event.asp";
    const DOC_TOPIC: &str = "Html DOM Events";

    let root = retrieve_document(&DOC_URL, DOC_TOPIC)?;

    let events = root.find(predicate::Name("table").and(predicate::Class("ws-table-all"))).next()
        .ok_or_else(|| anyhow!("Could not find expected contents of {DOC_TOPIC} - {DOC_URL}"))?
        .find(predicate::Name("tr")).skip(1);

    events.map(|row| {
        let mut columns = row.find(predicate::Name("td"));

        let (mut event, url) = parse_label_and_url(&columns.next(), UrlOption::Optional, BASE_JS_URL)
            .map_err(|err| {
                let html = row.html();

                anyhow!("Exception parsing event & link: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}\n  Err: {err}")
            })?;

        event.insert_str(0, EVENT_PREFIX);

        let details = match url {
            Some(url) => Some(scrape_event_details_page(&url, &event)?),
            None => None
        };

        let description = columns.next()
            .ok_or_else(|| {
                let html = row.html();

                anyhow!("Could not find {event:?} event's description: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}")
            })?.text();

        let (description, deprecated) = parse_event_details(&description);

        let objects = columns.next()
            .ok_or_else(|| {
                let html = row.html();

                anyhow!("Could not find {event:?} event's objects: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}")
            })?
            .find(predicate::Name("a"))
            .map(|obj| {
                let (event, url) = parse_label_and_url(&Some(obj), UrlOption::Required, BASE_JS_URL)?;

                Ok((event, url.unwrap()))
            }).collect::<Result<RawEventObjects>>()?;

        Ok((event, description, deprecated, details, objects))
    }).collect::<Result<RawEvents>>()
}

pub fn scrape_event_objects_page() -> Result<Vec<String>> {
    const DOC_URL: &str = "https://www.w3schools.com/jsref/dom_obj_event.asp";
    const DOC_TOPIC: &str = "Html DOM Events";

    let _root = retrieve_document(&DOC_URL, DOC_TOPIC)?;

    Ok(vec![])
}

pub fn scrape_tag_detail_pages(tags_by_category: &RawTagsByCategory) -> Result<RawTagsDetails> {
    tags_by_category.iter()
        .flat_map(|(_category, tags)|
            tags.iter()
                .filter_map(|(tag, url, (_desc, supported, _alt))| {
                    if *supported {
                        Some(Ok((
                            tag.clone(),
                            match scrape_tag_details_page(tag, url) {
                                Ok(details) => details,
                                Err(err) => return Some(Err(err))
                            }
                        )))
                    } else {
                        None
                    }
                })
        )
        .collect::<Result<RawTagsDetails>>()
}

fn scrape_event_details_page(url: &Url, event: &String) -> Result<(SupportedBrowsers, Map<String, String>)> {
    let doc_topic = format!("{event:?} Event Details");

    let root = retrieve_document(url, &doc_topic)?;

    let browser_support = root.find(predicate::Name("h2")).find(|nd| nd.text() == "Browser Support")
        .ok_or_else(|| anyhow!("Could not find browser support: {doc_topic} - {url}"))?;

    let err_details = {
        let url = url.as_str();

        move || format!("{doc_topic} - {url}")
    };

    let supported = parse_browser_support_details(&browser_support, &err_details)?;

    let details = root.find(predicate::Name("h2")).find(|nd| nd.text() == "Technical Details")
        .ok_or_else({
            let err_details = err_details();

            move || anyhow!("Could not find technical details: {err_details}")
        })?
        .iter_sibling(
            predicate::Name("table").and(predicate::Class("ws-table-all")), predicate::Name("hr"),
        ).next()
        .ok_or_else({
            let err_details = err_details();

            move || anyhow!("Could not find technical details table: {err_details}")
        })?;

    let rows = details.find(predicate::Name("tr")).filter(|nd| !nd.text().trim().is_empty());

    let details = rows
        .map(|row| {
            let label = row.find(predicate::Name("th")).next()
                .ok_or_else({
                    let html = row.html();
                    let err_details = err_details();

                    move || anyhow!("Could not parse details label: {err_details}\n  html: {html}")
                })?.text();

            let value = row.find(predicate::Name("td")).next()
                .ok_or_else({
                    let html = row.html();
                    let err_details = err_details();

                    move || anyhow!("Could not parse details value: {err_details}\n  html: {html}")
                })?.text();

            Ok((label, value))
        }).collect::<Result<RawTechDetails>>()?;

    Ok((supported, details))
}

fn parse_event_category(events: Node) -> Result<RawEventNames> {
    let rows = events.find(predicate::Name("tr")).skip(1);

    rows.map(|row| {
        let mut columns = row.find(predicate::Name("td"));

        let (mut event, _url) = parse_label_and_url(&columns.next(), UrlOption::Optional, BASE_TAGS_URL)?;

        if event.starts_with(EVENT_PREFIX) { event.insert_str(0, EVENT_PREFIX); }

        Ok(event)
    }).collect::<Result<RawEventNames>>()
}

fn parse_event_details(details: &str) -> (String, Option<AlternativeEvents>) {
    if details.starts_with("Deprecated.") {
        let alternatives = details.trim_start_matches("Deprecated. Use the ")
            .trim_end_matches(" event instead")
            .split(',')
            .map(|v| {
                let mut alt = v.trim().to_string();

                alt.insert_str(0, EVENT_PREFIX);

                alt
            })
            .collect::<AlternativeEvents>();

        (String::from("Deprecated"), Some(alternatives))
    } else {
        (details.to_string(), None)
    }
}

fn parse_tip_events<'a>(events: Node<'a>, category: &str) -> Result<Node<'a>> {
    events.iter_sibling(predicate::Name("div"), predicate::Name("br"))
        .next()
        .ok_or_else(|| anyhow!("Could not find {category:?} Tip Events"))?
        .find(predicate::Name("table").and(predicate::Class("ws-table-all")))
        .next()
        .ok_or_else(|| anyhow!("Could not find {category:?} Tip Events Table"))
}
