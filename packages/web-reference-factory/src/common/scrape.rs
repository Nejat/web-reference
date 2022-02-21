use std::str::FromStr;

use anyhow::Result;
use reqwest::Url;
use select::node::Node;
use select::predicate;
use select::predicate::Predicate;

use web_reference::prelude::*;

use crate::common::*;
#[cfg(feature = "build")]
use crate::ignored::IGNORED_SECTIONS;
use crate::types::*;

pub fn scrape_global_attributes_page() -> Result<RawGlobalAttributes> {
    const DOC_URL: &str = "https://www.w3schools.com/tags/ref_standardattributes.asp";
    const DOC_TOPIC: &str = "Global Attributes";

    let root = retrieve_document(&DOC_URL, DOC_TOPIC)?;

    let attributes = root.find(predicate::Class("w3-main")).next()
        .ok_or_else(|| anyhow!("Could not find expected contents of {DOC_TOPIC} - {DOC_URL}"))?
        .find(predicate::Class("ws-table-all")).next()
        .ok_or_else(|| anyhow!("Could not find table: {DOC_TOPIC} - {DOC_URL}"))?
        .find(predicate::Name("tr")).skip(1);

    attributes.map(|row| {
        let mut columns = row.find(predicate::Name("td"));

        let (attribute, url) = parse::parse_label_and_url(&columns.next(), UrlOption::Required, BASE_TAGS_URL)
            .map_err(|err| {
                let html = row.html();

                anyhow!("Exception parsing label and url: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}\n  Err: {err}")
            })?;

        let details = scrape_global_attribute_page(&url.unwrap(), &attribute)?;

        let description = if let Some(description) = columns.next() {
            description.text()
        } else {
            let html = row.html();

            bail!("Could not find {attribute:?} attribute's description: {DOC_TOPIC} - {DOC_URL}\n  html: {html:?}");
        };

        #[cfg(feature = "analyze")]
        add_attr_values(&attribute, &details.1);

        Ok((attribute, details, description))
    }).collect::<Result<RawGlobalAttributes>>()
}

pub fn scrape_tag_details_page(
    tag: &str,
    url: &Url,
    #[cfg(feature = "analyze")]
    sections: &mut DebugPageSections,
) -> Result<RawTagDetails> {
    let root = retrieve_document(url, &format!("{tag:?} Tag Details"))?;

    let headers = root.find(predicate::Name("h2"));

    let mut support = None;
    let mut attributes = None;
    let mut optional_attributes = None;
    let mut global_attributes = None;
    let mut events = None;

    for header in headers {
        let header_text = header.text();
        let header_text = header_text.trim();
        let err_details = {
            let url = url.as_str();

            move || format!("<{tag}> tag - {url}")
        };

        match header_text {
            "Browser Support" => {
                support = Some(parse::parse_browser_support_details(&header, &err_details)?);
            }
            "Attributes" => {
                attributes = Some(scrape_tag_attributes(&header, tag, &err_details)?);
            }
            "Global Attributes" |
            "Global Attributes and Events" |
            "Standard Attributes" => {
                global_attributes = Some(parse::parse_standard_attributes(&header, &err_details)?);
            }
            "Event Attributes" => {
                events = Some(parse::parse_standard_attributes(&header, &err_details)?);
            }
            "Optional Attributes" => {
                optional_attributes = Some(scrape_tag_attributes(&header, tag, &err_details)?);
            }
            #[cfg(feature = "build")]
            section if IGNORED_SECTIONS.contains_key(section) &&
                IGNORED_SECTIONS[section].contains(format!("<{tag}>").as_str()) => {}
            #[cfg(feature = "build")]
            header => bail!("Unexpected section {header:?} in <{tag:?}> details - {url}"),
            #[cfg(feature = "analyze")]
            header => {
                if !sections.contains_key(header) {
                    sections.insert(header.to_string(), vec![]);
                }

                sections.get_mut(header).unwrap().push(tag.to_string());
            }
        }
    }

    Ok((support, attributes, events, global_attributes, optional_attributes))
}

// scrape tags grouped by category
pub fn scrape_tags_by_category_page() -> Result<RawTagsByCategory> {
    const DOC_URL: &str = "https://www.w3schools.com/tags/ref_byfunc.asp";
    const DOC_TOPIC: &str = "Tags by Category";

    let root = retrieve_document(&DOC_URL, DOC_TOPIC)?;

    let main = root.find(predicate::Class("w3-main")).next()
        .ok_or_else(|| anyhow!("Could not find expected contents of {DOC_TOPIC} - {DOC_URL}"))?;

    // iterate all categories with table of defined tags
    main.find(predicate::Name("h2"))
        .filter_map(
            |category|
                category
                    .iter_sibling(predicate::Name("table"), predicate::Name("hr"))
                    .next()
                    .map(|table| (category, table))
        )
        // iterate all tags in a category
        .map(|(category, table)| {
            let tag_category = category.text();
            let rows = table.find(predicate::Name("tr")).skip(1)
                // filter out comment tag
                .filter(|row| !row.text().contains("<!"));

            let tags = rows.map(|row| {
                let mut columns = row.find(predicate::Name("td"));

                let (tag, url) = parse::parse_label_and_url(&columns.next(), UrlOption::Required, BASE_TAGS_URL)
                    .map_err(|err| anyhow!("Exception parsing tag url: {DOC_TOPIC} - {DOC_URL}\n  Err: {err}"))?;

                let details = if let Some(details) = columns.next() {
                    parse::parse_tag_details(&details.text().replace(' ', ""), &tag)
                        .map_err(|err| anyhow!("Exception parsing tag details: {DOC_TOPIC} - {DOC_URL}\n  Err: {err}"))?
                } else {
                    let html = row.html();

                    bail!("Could not find {tag:?} tag's details: {DOC_TOPIC} {DOC_URL}\n  html: {html:?}");
                };

                Ok((tag, url.unwrap(), details))
            }).collect::<Result<RawTags>>();

            let tags = match tags {
                Ok(tags) => tags.into_iter()
                    .flat_map(|tag| {
                        // convert <tagN1> to <tagN2> tags into individual tags
                        if tag.0.contains("> to <") {
                            let (_match, tag_prefix, start, end) = regex_captures!(r#"^(\w+)(\d+)> to <\w+(\d+)$"#, &tag.0).unwrap();

                            let start = usize::from_str(start).unwrap();
                            let end = usize::from_str(end).unwrap() + 1;

                            (start..end)
                                .map(|idx| {
                                    let mut new_tag = tag.clone();

                                    new_tag.0 = format!("{tag_prefix}{idx}");

                                    new_tag
                                }).collect()
                        } else {
                            vec![tag]
                        }
                    }).collect(),
                Err(err) => bail!("Exception parsing tags: {DOC_TOPIC} - {DOC_URL}\n  Err: {err}")
            };

            Ok((tag_category, tags))
        }).collect::<Result<RawTagsByCategory>>()
}

pub fn parse_browser_support(row: Node) -> Result<(String, BrowsersSupported)> {
    let mut columns = row.find(predicate::Name("td")).map(|e| e.text());

    let label = if let Some(label) = columns.next() {
        label.trim().trim_start_matches('<').trim_end_matches('>').to_string()
    } else {
        let html = row.html();

        bail!("Could not find browser support label\n  html: {html:?}");
    };

    let chrome = if let Some(support) = columns.next() {
        Support::from_str(&support)
            .map_err(|err| anyhow!("Exception parsing Chrome support:\n  Err: {err}"))?
    } else {
        let html = row.html();

        bail!("Could not find {label:?} Chrome support\n  html: {html:?}");
    };

    let edge = if let Some(support) = columns.next() {
        Support::from_str(&support)
            .map_err(|err| anyhow!("Exception parsing Edge support:\n  Err: {err}"))?
    } else {
        let html = row.html();

        bail!("Could not find {label:?} Edge support\n  html: {html:?}");
    };

    let firefox = if let Some(support) = columns.next() {
        Support::from_str(&support)
            .map_err(|err| anyhow!("Exception parsing Firefox support:\n  Err: {err}"))?
    } else {
        let html = row.html();

        bail!("Could not find {label:?} Firefox support\n  html: {html:?}");
    };

    let safari = if let Some(support) = columns.next() {
        Support::from_str(&support)
            .map_err(|err| anyhow!("Exception parsing Safari support:\n  Err: {err}"))?
    } else {
        let html = row.html();

        bail!("Could not find {label:?} Safari support\n  html: {html:?}");
    };

    let opera = if let Some(support) = columns.next() {
        Support::from_str(&support)
            .map_err(|err| anyhow!("Exception parsing Opera support:\n  Err: {err}"))?
    } else {
        let html = row.html();

        bail!("Could not find {label:?} Opera support\n  html: {html:?}");
    };

    let browsers = vec![
        (Browser::Chrome, chrome),
        (Browser::Edge, edge),
        (Browser::Firefox, firefox),
        (Browser::Safari, safari),
        (Browser::Opera, opera),
    ].into_iter().collect();

    Ok((label, browsers))
}

fn scrape_global_attribute_page(url: &Url, attribute: &str) -> Result<RawGlobalAttributeDetails> {
    let doc_topic = format!("Global {attribute:?} Attribute");
    let root = retrieve_document(url, &doc_topic)?;

    let support = root.find(predicate::Name("table").and(predicate::Class("browserref"))).next()
        .ok_or_else(|| anyhow!("Could not find browser support table: {doc_topic} - {url}"))?;

    let supported = support.find(predicate::Name("tr")).nth(1)
        .ok_or_else(|| anyhow!("Expected at lease one one browser support definition: {doc_topic} - {url}"))?;

    let (_, supported) = parse_browser_support(supported)?;
    let caveats = parse::parse_caveats(&support);

    let values = root.find(predicate::Name("h2")).find(|nd| nd.text() == "Attribute Values");

    let attribute_values = match values {
        Some(values) => {
            let err_details = || format!("{doc_topic} - {url}");

            parse::parse_attribute_values(&values, &err_details)?
        }
        None => AttributeValues::default()
    };

    Ok((supported, attribute_values, caveats))
}

fn scrape_tag_attributes(header: &Node, tag: &str, err_details: &impl Fn() -> String) -> Result<TagAttributes> {
    let attributes = header.iter_sibling(predicate::Name("table"), predicate::Name("hr")).next();

    attributes.map_or_else(
        || {
            let header_text = header.text();
            let err_details = err_details();

            Err(anyhow!("Could not find {header_text:?} table: {err_details}"))
        },
        |attributes| {
            let rows = attributes.find(predicate::Name("tr")).skip(1);

            rows.into_iter()
                .map(|row| {
                    let mut columns = row.find(predicate::Name("td"));

                    let (name, url) = parse::parse_label_and_url(&columns.next(), UrlOption::Optional, BASE_TAGS_URL)
                        .map_err(|err| {
                            let html = row.html();
                            let err_details = err_details();

                            anyhow!("Exception parsing attribute: {err_details}\n html: {html:?}\n  Err: {err}")
                        })?;

                    let (support, attribute_values) = match url.map(|url| scrape_tag_attributes_page(&url, tag, &name)) {
                        Some(Ok(tag_attributes)) => tag_attributes,
                        Some(Err(err)) => {
                            let header_text = header.text();
                            let err_details = err_details();

                            bail!("Exception parsing {name:?} tag attribute for {header_text:?}: {err_details}\n  Err: {err}")
                        }
                        None => (None, None),
                    };

                    let supported = support.unwrap_or_default();

                    let tag_attribute_values = columns.next();

                    let values = match attribute_values {
                        Some(values) => values,
                        None => {
                            if let Some(values) = tag_attribute_values {
                                values.text().split('\n')
                                    .filter_map(|value| {
                                        let v = value.trim();

                                        if v.is_empty() {
                                            None
                                        } else {
                                            Some((v.to_string(), Value::default()))
                                        }
                                    }).collect::<AttributeValues>()
                            } else {
                                let header_text = header.text();
                                let err_details = err_details();

                                bail!("Expected values for {header_text:?} attribute {name:?}: {err_details}");
                            }
                        }
                    };

                    let description = columns.next()
                        .ok_or_else(|| {
                            let header_text = header.text();
                            let err_details = err_details();

                            anyhow!("Expected description for {header_text:?} attribute {name:?}: {err_details}")
                        })?
                        .text().replace(' ', "");

                    #[cfg(feature = "analyze")]
                    add_attr_values(&name, &values);

                    let attribute = Attribute {
                        name: name.clone(),
                        belongs_to: AttributeBelongsTo::Tags(vec![tag.to_string()].into_iter().collect()),
                        description: Description::from(description),
                        supported,
                        values: AttributeValue::try_from(values)
                            .map_err(|err| anyhow!("Exception parsing attribute values\n  Err: {err}"))?,
                    };

                    Ok((name, attribute))
                }).collect::<Result<TagAttributes>>()
        })
}

fn scrape_tag_attributes_page(url: &Url, tag: &str, attr: &str) -> Result<(Option<SupportedBrowsers>, Option<AttributeValues>)> {
    let root = retrieve_document(url, &format!("<{tag}> {attr:?} Attributes"))?;

    let headers = root.find(predicate::Name("h2"));

    let mut support = None;
    let mut values = None;
    let err_details = {
        let url = url.as_str();

        move || format!("<{tag}> tag {attr:?} - {url}")
    };

    for header in headers {
        let header_text = header.text();
        let header_text = header_text.trim();

        match header_text {
            "Browser Support" => {
                support = Some(parse::parse_browser_support_details(&header, &err_details)?);
            }
            "Attribute Values" => {
                values = Some(parse::parse_attribute_values(&header, &err_details)?);
            }
            _ => {}
        }
    }

    Ok((support, values))
}
