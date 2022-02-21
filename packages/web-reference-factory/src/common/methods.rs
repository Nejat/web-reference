use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use reqwest::{IntoUrl, Url};
use select::document::Document;

#[cfg(feature = "analyze")]
use web_reference::prelude::*;

use crate::common::*;
use crate::REFRESH;

#[cfg(feature = "analyze")]
lazy_static! {
    ///
    static ref ATTR_VALUES: parking_lot::Mutex<Set<String>> = parking_lot::Mutex::new(<Set<String>>::new());
}

#[inline]
pub fn absolute_url(base: &str, address: &str) -> Result<Url> {
    Ok(Url::parse(&format!("{base}{address}"))?)
}

#[cfg(feature = "analyze")]
pub fn add_attr_values(attribute: &str, values: &AttributeValues) {
    if values.len() > 2 && values.iter().all(|(_ky, val)| val.name != "auto") {
        return
    }

    let mut attr_values = ATTR_VALUES.lock();

    let mut attr_values_set = values.keys().cloned().collect::<Vec<_>>().join(", ");

    if attr_values_set.is_empty() {
        attr_values_set = attribute.to_string();
    }

    if attr_values_set.trim() == attribute {
        attr_values_set.insert_str(0, "*")
    }

    attr_values.insert(attr_values_set);
}

#[cfg(feature = "analyze")]
pub fn list_attr_values() {
    let values = ATTR_VALUES.lock();

    for value in values.iter() {
        println!("{value:?}");
    }
}

pub fn retrieve_document<U: IntoUrl + Clone>(url: &U, doc: &str) -> Result<Document> {
    let url = url.clone().into_url()?;

    if url.as_str() == BASE_TAGS_URL { bail!("Unsupported url for {doc:?} - {url}") }

    let file_name = url.path_segments()
        .ok_or_else(|| anyhow!("Could not extract file name from url {url}"))?
        .last().unwrap().split('.').next().unwrap();

    let mut file_path = PathBuf::from(OFFLINE_PATH);

    file_path.push(file_name);
    file_path.set_extension("html");

    let text = if REFRESH || !file_path.exists() {
        let response = reqwest::blocking::get(url.clone())
            .map_err(|err| anyhow!("GET {doc} document failed - {url}\nErr: {err}"))?;

        let status = response.status();

        ensure!(status.is_success(), "{doc} document request failed - {url}, Status Code: {status}");

        let text = response.text()
            .map_err(|err| anyhow!("{doc} document text retrieval failed - {url}\nErr: {err}"))?;

        if !file_name.is_empty() {
            fs::create_dir_all(OFFLINE_PATH)?;
            fs::write(file_path, &text)?;
        }

        text
    } else {
        fs::read_to_string(file_path)?
    };

    Ok(Document::from(text.as_str()))
}

// use for debugging
#[allow(dead_code)]
pub fn write_debug(content: &str) {
    use std::fs::OpenOptions;
    use std::io::Write;
    use crate::DEBUG_TEXT_FILE;

    let mut file = OpenOptions::new()
        .create(true).append(true)
        .open(DEBUG_TEXT_FILE)
        .expect("expect to create or open debug output");

    file.write_all(content.as_bytes()).expect("expect to write debug");
}
