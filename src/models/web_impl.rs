use crate::prelude::*;

const EVENT_PREFIX: &str = "on";

impl WebReference {
    ///
    #[must_use]
    pub fn is_valid_attribute(&self, attribute: &str) -> bool {
        self.attributes.contains_key(attribute)
    }

    ///
    #[must_use]
    pub fn is_valid_event(&self, event: &str) -> bool {
        if event.starts_with(EVENT_PREFIX) {
            self.events.contains_key(event)
        } else {
            self.events.contains_key(&format!("{EVENT_PREFIX}{event}"))
        }
    }

    ///
    #[must_use]
    pub fn is_valid_tag(&self, tag: &str) -> bool {
        self.tags.contains_key(tag)
    }

    ///
    #[must_use]
    pub fn get_attribute(&self, attribute: &str) -> Option<&Attribute> {
        let attributes = self.attributes.get(attribute)?;

        if attributes.len() == 1 {
            attributes.iter().next().map(|(_key, value)| value)
        } else {
            None
        }
    }

    ///
    #[must_use]
    pub fn get_attribute_category(&self, category: AttributeCategory) -> Option<&AttributeCategoryDetails> {
        self.attributes_categorized.get(&category)
    }

    ///
    #[must_use]
    pub fn get_attributes(&self, attribute: &str) -> Option<&TagAttributes> {
        self.attributes.get(attribute)
    }

    ///
    #[must_use]
    pub fn get_attributes_of_category(&self, category: AttributeCategory) -> Option<Vec<&Attribute>> {
        Some(
            self.attributes_categorized.get(&category)?
                .attributes.iter()
                .filter_map(|attribute| self.get_attribute(attribute))
                .collect()
        )
    }

    ///
    #[must_use]
    pub fn get_event(&self, event: &str) -> Option<&Event> {
        self.events.get(event)
    }

    ///
    #[must_use]
    pub fn get_event_category(&self, category: EventCategory) -> Option<&EventCategoryDetails> {
        self.events_categorized.get(&category)
    }

    ///
    #[must_use]
    pub fn get_events_of_category(&self, category: EventCategory) -> Option<Vec<&Event>> {
        Some(
            self.events_categorized.get(&category)?
                .events.iter()
                .filter_map(|event| self.get_event(event))
                .collect()
        )
    }

    ///
    #[must_use]
    pub fn get_tag(&self, tag: &str) -> Option<&Tag> {
        self.tags.get(tag)
    }

    ///
    #[must_use]
    pub fn get_tag_attribute(&self, attribute: &str, tag: &Tag) -> Option<&Attribute> {
        self.attributes.get(attribute)?.get(&tag.name)
    }

    ///
    #[must_use]
    pub fn get_tag_attributes(&self, tag: &Tag) -> Option<Vec<&Attribute>> {
        Some(
            tag.attributes.iter()
                .filter_map(|attr| self.get_attribute(attr))
                .collect()
        )
    }

    ///
    #[must_use]
    pub fn get_tag_category(&self, category: TagCategory) -> Option<&TagCategoryDetails> {
        self.tags_categorized.get(&category)
    }

    ///
    #[must_use]
    pub fn get_tags_of_category(&self, category: TagCategory) -> Option<Vec<&Tag>> {
        Some(
            self.tags_categorized.get(&category)?
                .tags.iter()
                .filter_map(|tag| self.get_tag(tag))
                .collect()
        )
    }
}