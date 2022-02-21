use anyhow::Result;
use reqwest::Url;
use select::node::Node;
use select::predicate;

use web_reference::prelude::*;

use crate::common::*;
use crate::types::*;

pub fn parse_attribute_values(header: &Node, err_details: &impl Fn() -> String) -> Result<AttributeValues> {
    let attribute_values = header.iter_sibling(predicate::Name("table"), predicate::Name("hr")).next();

    attribute_values.map_or_else(
        || {
            let err_details = err_details();

            Err(anyhow!("Could not find table of attribute values: {err_details}"))
        },
        |table| {
            let rows = table.find(predicate::Name("tr")).skip(1);

            rows.into_iter()
                .map(|row| {
                    let mut columns = row.find(predicate::Name("td"));

                    let name = if let Some(name) = columns.next() {
                        name.text()
                    } else {
                        let err_details = err_details();

                        bail!("Could not find attribute value: {err_details}");
                    };

                    let description = if let Some(description) = columns.next() {
                        description.text().replace(' ', "")
                    } else {
                        let err_details = err_details();

                        bail!("Could not find  {name:?} attribute's description: {err_details}");
                    };

                    Ok((name.clone(), Value { name, description: Description::from(description) }))
                }).collect::<Result<AttributeValues>>()
        })
}

pub fn parse_caveats(node: &Node) -> RawCaveats {
    node
        .iter_sibling(predicate::Name("p"), predicate::Name("hr"))
        .map(|nd| nd.text())
        .collect::<RawCaveats>()
}

pub fn parse_browser_support_details(header: &Node, err_details: &impl Fn() -> String) -> Result<SupportedBrowsers> {
    let mut table = if let Some(table) = header.parent() {
        table.find(predicate::Class("browserref"))
    } else {
        let header_text = header.text();
        let err_details = err_details();

        bail!("Parent node expected for {header_text}: {err_details}");
    };

    let table = table.next()
        .ok_or_else(|| {
            let header_text = header.text();
            let err_details = err_details();

            anyhow!("Could not find {header_text:?} browser support: {err_details}")
        })?;

    let mut rows = table.find(predicate::Name("tr"));
    let mut next = || rows.next().ok_or_else(|| {
        let header_text = header.text();
        let err_details = err_details();

        anyhow!("Could not scrape {header_text:?} browser support: {err_details}")
    });

    next()?;

    let (_label, support) = parse_browser_support(next()?)?;
    let caveats = parse_caveats(&table).into_iter().map(Description::from).collect();

    Ok(SupportedBrowsers { supported: support, caveats })
}

// parse an anchor's test as a label and href as an url link
pub fn parse_label_and_url(node: &Option<Node>, urk_option: UrlOption, base: &str) -> Result<(String, Option<Url>)> {
    node.map_or_else(
        || Err(anyhow!("Expected a Node to parse label and url link")),
        |elm| Ok((
            // use element text as label
            elm.text().trim()
                .trim_start_matches('<')
                .trim_end_matches('>')
                .to_string(),
            // extract href of anchor element as url
            if urk_option == UrlOption::Required {
                Some(parse_tag_link(&elm, base)?)
            } else {
                parse_tag_link(&elm, base).ok()
            }
        )),
    )
}

pub fn parse_standard_attributes(header: &Node, err_details: &impl Fn() -> String) -> Result<bool> {
    let attributes = header.iter_sibling(predicate::Name("p"), predicate::Name("hr")).next();

    attributes.map_or_else(
        || {
            let header_text = header.text();
            let err_details = err_details();

            Err(anyhow!("Could not find {header_text:?} definition: {err_details}"))
        },
        |definition| {
            let definition = definition.text().replace('\n', "");
            let definition = definition.trim();

            if regex_is_match!("^The .+ tags?( also)? supports the (Global|Event) Attributes in HTML.$", definition) {
                Ok(true)
            } else if regex_is_match!("^The .+ tag does not support any (event|standard) attributes.$", definition) {
                Ok(false)
            } else {
                let header_text = header.text();
                let err_details = err_details();

                Err(anyhow!("Unexpected {header_text:?} definition {definition:?}: {err_details}"))
            }
        })
}

// parse suggested alternatives of unsupported tags
fn parse_tag_alternative(source: &[&str]) -> Result<TagAlternatives> {
    const NOT_SUPPORTED: &str = "Not supported in HTML5";
    const SUGGESTION_PREFIX: &str = "Use ";
    const SUGGESTION_SUFFIX: &str = " instead";

    if source[0] == NOT_SUPPORTED {
        let suggestion = source.get(1).unwrap_or(&"")
            .trim_start_matches(SUGGESTION_PREFIX)
            .trim_end_matches(SUGGESTION_SUFFIX).trim();

        Ok(match suggestion {
            "" => TagAlternatives::None,
            "CSS" => TagAlternatives::Css,
            tags => TagAlternatives::Tags(
                tags.split(" or ").map(|v|
                    v.trim()
                        .trim_start_matches('<')
                        .trim_end_matches('>')
                        .to_string()
                ).collect()
            )
        })
    } else {
        Err(anyhow!("Unexpected tag alternative statement: {source:?}"))
    }
}

// extract if tag is supported, any alternative if not supported and description of tag
pub fn parse_tag_details(source: &str, tag: &str) -> Result<RawTag> {
    let source = source.replace('\n', "").replace(' ', "");

    // split into sentences
    let mut parts = source.split('.')
        .filter_map(|v| {
            let trimmed = v.trim();

            if trimmed.is_empty() { None } else { Some(trimmed) }
        }).collect::<Vec<_>>();

    Ok(match parts.len() {
        // single sentence describes tag
        1 => (
            Description::from(parts.pop().unwrap().trim()),
            true,
            TagAlternatives::None
        ),
        // two or three sentences indicate unsupported tag and suggested alternatives
        2 | 3 => (
            Description::from(parts.pop().unwrap().trim()),
            false,
            parse_tag_alternative(&parts)?
        ),
        _unsupported => bail!("Unexpected <{tag}> tag definition: {source:?}")
    })
}

// find first anchor element as extract href as url
fn parse_tag_link(source: &Node, base: &str) -> Result<Url> {
    let tag = if source.is(predicate::Name("a")) {
        *source
    } else {
        source.find(predicate::Name("a")).next()
            .ok_or_else(|| anyhow!("Missing expected anchor tag"))?
    };

    absolute_url(
        base,
        tag.attr("href")
            .ok_or_else(|| anyhow!("Anchor tag missing required href attribute"))?,
    )
}
