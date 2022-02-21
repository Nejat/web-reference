use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Result;

use crate::common::*;
use crate::types::*;

type Sections = Vec<(String, Vec<String>)>;

pub fn run_factory() -> Result<()> {
    let tags_category = scrape_tags_by_category_page()?;
    let sections = scrape_tag_detail_sections(&tags_category);

    scrape_global_attributes_page()?;

    list_attr_values();

    write_ignored_rust_code(sections)
}

fn scrape_tag_detail_sections(tags_by_category: &RawTagsByCategory) -> Sections {
    let mut sections = DebugPageSections::default();

    tags_by_category.iter()
        .map(|(_category, tags)|
            tags.iter()
                .filter_map(|(tag, url, (_desc, supported, _alt))| {
                    if *supported {
                        Some(scrape_tag_details_page(tag, url, &mut sections).unwrap())
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
        ).for_each(drain);

    let mut sections = sections.into_iter().collect::<Vec<_>>();

    let max_tags = sections.iter().map(|(_header, tags)| tags.len()).max().unwrap();

    sections.sort_by_key(|(header, tags)| (max_tags - tags.len(), header.clone()));

    return sections;

    fn drain<T>(_: T) {}
}

fn write_ignored_rust_code(sections: Sections) -> Result<()> {
    let ignored = File::create("packages/web-reference-factory/src/ignored.rs")?;
    let mut out = BufWriter::new(ignored);

    writeln!(out, "use std::collections::{{HashMap, HashSet}};")?;
    writeln!(out)?;
    writeln!(out, "lazy_static! {{")?;
    writeln!(out, "    pub static ref IGNORED_SECTIONS: HashMap<&'static str, HashSet<&'static str>> = hashmap! {{")?;

    for (section, tags) in sections {
        const LINE_LEADER: &str = "           ";

        writeln!(out, "        {section:?} => hashset! {{")?;
        write!(out, "{LINE_LEADER}")?;

        let mut line_len = LINE_LEADER.len();

        for tag in tags {
            if line_len >= 70 {
                writeln!(out)?;
                write!(out, "{LINE_LEADER}")?;
                line_len = LINE_LEADER.len();
            } else {
                line_len += tag.len() + 6;
            }
            write!(out, r#" "<{tag}>","#)?;
        }

        writeln!(out)?;
        writeln!(out, "        }},")?;
    }

    writeln!(out, "    }};")?;
    writeln!(out, "}}")?;

    Ok(())
}
