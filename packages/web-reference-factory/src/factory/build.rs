use anyhow::Result;
use itertools::Itertools;

use web_reference::prelude::*;

use crate::types::*;

pub fn build_attributes(
    attributes: RawAttributes,
    global_attributes: RawGlobalAttributes,
    tag_details: RawTagsDetails,
    attributes_categorized: &AttributesCategorized,
) -> Result<Attributes> {
    let global = attributes_categorized.get(&AttributeCategory::GlobalAttributes).unwrap();
    let not_supported = attributes_categorized.get(&AttributeCategory::NotSupported).unwrap();

    let attributes = attributes.into_iter()
        .map(|(name, _url, _belongs_to, _desc)| name)
        .collect::<Set<String>>();

    let mut tag_attributes = collect_tag_attributes(tag_details);

    {
        let unhandled = attributes.iter()
            .filter(
                |attr| !tag_attributes.contains_key(*attr) &&
                    !global.attributes.contains(*attr) &&
                    !not_supported.attributes.contains(*attr)
            ).collect::<Set<_>>();

        if !unhandled.is_empty() {
            bail!("Unhandled attributes {:#?}", unhandled);
        }
    }

    #[cfg(feature = "analyze")]
    {
        let additional = tag_attributes.iter()
            .filter_map(|(attr, attrs)| if attributes.contains(attr) {
                None
            } else {
                Some((attr, attrs.keys().map(|tag| format!("<{tag}>")).collect::<Set<_>>()))
            }).collect::<Map<_, _>>();

        if !additional.is_empty() {
            println!("Unlisted attributes {:#?}", additional);
        }
    }

    for (name, (supported, values, caveats), desc) in global_attributes {
        let attribute = Attribute {
            name,
            belongs_to: AttributeBelongsTo::Global,
            description: Description::from(desc.as_str()),
            supported: SupportedBrowsers {
                supported,
                caveats: caveats.into_iter().map(From::from).collect(),
            },
            values: AttributeValue::try_from(values)
                .map_err(|err| anyhow!("Exception parsing global attribute values\n  Err: {err}"))?,
        };

        tag_attributes.insert(
            attribute.name.clone(),
            [(String::from("global"), attribute)].into_iter().collect(),
        );
    }

    Ok(tag_attributes)
}

pub fn build_events(events: RawEvents, events_attrs: RawAttributes) -> Result<Events> {
    let event_belongs_to = events_attrs.into_iter()
        .map(|(event, _url, belongs_to, _desc)| {
            (event, belongs_to)
        }).collect::<Map<String, RawBelongsTo>>();

    events.into_iter()
        .map(|(event, desc, deprecated, details, objects)| {
            let (supported, details) = if let Some((supported, tech_details)) = details {
                (
                    supported,
                    parse_event_details(&tech_details)
                        .map_err(|err| anyhow!("Exception parsing {event:?} event details\n  Err: {err}"))?
                )
            } else {
                (
                    SupportedBrowsers::default(),
                    EventDetails { bubbles: false, cancelable: false }
                )
            };

            let belongs_to = match event_belongs_to.get(&event) {
                Some(raw_belongs_to) => parse_event_belongs_to(raw_belongs_to)?,
                None => EventBelongsTo::NotDefined
            };

            let event_objects = objects.into_iter().map(|(obj, _url)| obj).collect();

            let event = Event {
                name: event,
                belongs_to,
                details,
                description: Description::from(desc),
                event_objects,
                supported,
                deprecated: deprecated.unwrap_or_default(),
            };

            Ok((event.name.clone(), event))
        })
        .collect::<Result<Events>>()
}

fn parse_event_details(details: &RawTechDetails) -> Result<EventDetails> {
    let bubbles = details.get("Bubbles:")
        .ok_or_else(|| anyhow!(r#"Could not find "Bubbles" event detail"#))? == "No";
    let cancelable = details.get("Cancelable:")
        .ok_or_else(|| anyhow!(r#"Could not find "Cancelable" event detail"#))? == "No";

    Ok(EventDetails {
        bubbles,
        cancelable,
    })
}

pub fn build_supported_browsers(supported: RawBrowserSupport) -> TagsSupport {
    supported.into_iter()
        .map(|(tag, browsers, attributes, caveats)| {
            let mut attributes = attributes.into_iter().collect::<Map<_, _>>();

            attributes.sort_keys();

            let tag_support = TagsSupportedBrowsers {
                tag: tag.clone(),
                browsers: SupportedBrowsers {
                    supported: browsers,
                    caveats: caveats.into_iter().map(From::from).collect(),
                },
                attributes,
            };

            (tag, tag_support)
        }).collect::<TagsSupport>()
}

pub fn build_tags(tags: &RawTagsByCategory, details: &RawTagsDetails) -> Result<Tags> {
    tags.iter()
        .flat_map(|(_category, tags)| tags)
        .into_group_map_by(|(tag_name, _, _)| tag_name.clone()).into_iter()
        .map(|(_tag, tags)| {
            let (name, _, (description, _, alts)) =
                tags.into_iter()
                    .fold(None, |acc, (tag, url, (desc, supported, alts))| {
                        match acc {
                            None => Some(Ok((tag, url, (desc.clone(), supported, alts)))),
                            Some(Ok((_, url1, (desc1, _, alts1)))) =>
                                if url != url1 {
                                    Some(Err(anyhow!("<{tag}> urls {url} != {url1}")))
                                } else if alts == alts1 {
                                    Some(Ok((tag, url, (desc.combine(&desc1), supported, alts))))
                                } else {
                                    Some(Err(anyhow!("<{tag}> alternatives {alts:?} != {alts1:?}")))
                                },
                            err @ Some(Err(_)) => err
                        }
                    }).unwrap()?;

            let (supported, attributes, event_attributes, global_attributes, optional_attributes) =
                details.get(name).unwrap_or(&(None, None, None, None, None));

            let supported = supported.clone().unwrap_or_default();

            let attributes = attributes.clone().unwrap_or_default().into_iter()
                .map(|(key, _value)| key)
                .collect();

            let optional_attributes = optional_attributes.clone().unwrap_or_default().into_iter()
                .map(|(key, _value)| key)
                .collect();

            let global_attributes = (*global_attributes).unwrap_or_default();
            let global_events = (*event_attributes).unwrap_or_default();

            let tag = Tag {
                name: name.clone(),
                description,
                supported,
                attributes,
                optional_attributes,
                global_attributes,
                global_events,
                alternatives: alts.clone(),
            };

            Ok((name.clone(), tag))
        }).collect::<Result<Tags>>()
}

pub fn categorize_attributes(attributes: &RawAttributes) -> Result<AttributesCategorized> {
    Ok(attributes.iter()
        .map(|(attr, _url, belongs_to, _desc)| {
            let belongs_to = parse_attribute_belongs_to(belongs_to)?;
            let category = AttributeCategory::from(belongs_to);

            Ok((category, attr))
        })
        .collect::<Result<Vec<_>>>()?.into_iter()
        .into_group_map().into_iter()
        .map(|group| {
            let (key, attrs) = group;

            let mut category = AttributeCategoryDetails {
                category: key,
                attributes: attrs.into_iter().cloned().collect(),
            };

            category.attributes.sort();

            (key, category)
        }).collect::<AttributesCategorized>())
}

pub fn categorize_events(events_by_category: RawEventsByCategory) -> Result<EventsCategorized> {
    events_by_category.into_iter()
        .map(|(category, (desc, events))| {
            let category = parse_event_category(&category)?;

            let mut category_details = EventCategoryDetails {
                category,
                description: Description::from(desc.unwrap_or_default()),
                events: events.iter().cloned().collect(),
            };

            category_details.events.sort();

            Ok((category, category_details))
        }).collect::<Result<EventsCategorized>>()
}

pub fn categorize_tags(tags_by_category: &RawTagsByCategory) -> Result<TagsCategorized> {
    tags_by_category.iter()
        .map(|(category, tags)| {
            let category = parse_tag_category(category)?;

            let tag_category = TagCategoryDetails {
                category,
                tags: tags.iter()
                    .filter_map(
                        |(tag, _, (_, supported, _))| if *supported { Some(tag.clone()) } else { None }
                    ).collect(),
            };

            Ok((category, tag_category))
        }).collect::<Result<TagsCategorized>>()
}

fn collect_tag_attributes(tag_details: RawTagsDetails) -> Attributes {
    let mut attrs = tag_details.into_iter()
        .flat_map(|(tag, (_supported, attributes, _events, _global_attrs, optional_attrs))| {
            let attributes = attributes.unwrap_or_default().into_iter()
                .map(|(name, attr)| (name, (tag.clone(), attr)));

            let optional = optional_attrs.unwrap_or_default().into_iter()
                .map(|(name, attr)| (name, (tag.clone(), attr)));

            attributes.chain(optional)
                .collect::<Vec<(String, (String, Attribute))>>()
        })
        .into_group_map().into_iter()
        .map(|(key, attrs)| {
            let mut attrs = attrs.into_iter().collect::<Map<_, _>>();

            attrs.sort_keys();

            (key, attrs)
        }).collect::<Attributes>();

    attrs.sort_keys();

    attrs
}

fn parse_attribute_belongs_to(belongs_to: &RawBelongsTo) -> Result<AttributeBelongsTo> {
    Ok(match belongs_to {
        RawBelongsTo::Global => AttributeBelongsTo::Global,
        RawBelongsTo::NotSupported => AttributeBelongsTo::NotSupported,
        RawBelongsTo::Tags(tags) => AttributeBelongsTo::Tags(tags.clone()),
        RawBelongsTo::AllVisible => bail!(r#"Unexpected attribute "AllVisible" belongs to"#)
    })
}

fn parse_event_belongs_to(belongs_to: &RawBelongsTo) -> Result<EventBelongsTo> {
    Ok(match belongs_to {
        RawBelongsTo::AllVisible => EventBelongsTo::AllVisible,
        RawBelongsTo::Tags(tags) => EventBelongsTo::Tags(tags.clone()),
        belongs_to => bail!("Unexpected tag belongs to {belongs_to:?}")
    })
}

fn parse_event_category(category: &str) -> Result<EventCategory> {
    Ok(match category {
        "Window Event Attributes" => EventCategory::WindowEvents,
        "Form Events" => EventCategory::FormEvents,
        "Keyboard Events" => EventCategory::KeyboardEvents,
        "Mouse Events" => EventCategory::MouseEvents,
        "Drag Events" => EventCategory::DragEvents,
        "Clipboard Events" => EventCategory::ClipboardEvents,
        "Media Events" => EventCategory::MediaEvents,
        "Misc Events" => EventCategory::MiscellaneousEvents,
        category => bail!("Unexpected event category {category:?}")
    })
}

fn parse_tag_category(src: &str) -> Result<TagCategory> {
    Ok(match src {
        "Basic HTML" => TagCategory::Basic,
        "Formatting" => TagCategory::Formatting,
        "Forms and Input" => TagCategory::FormsInput,
        "Frames" => TagCategory::Frames,
        "Images" => TagCategory::Images,
        "Audio / Video" => TagCategory::AudioVideo,
        "Links" => TagCategory::Links,
        "Lists" => TagCategory::Lists,
        "Tables" => TagCategory::Tables,
        "Styles and Semantics" => TagCategory::StylesSemantics,
        "Meta Info" => TagCategory::Meta,
        "Programming" => TagCategory::Programming,
        src => bail!("{src:?} is not a valid tag category")
    })
}

#[allow(clippy::unnecessary_wraps)] // future impl
pub fn event_objects(_objects: &[String]) -> Result<EventObjects> {
    Ok(EventObjects::default())
}