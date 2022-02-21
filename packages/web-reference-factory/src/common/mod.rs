pub use crate::common::methods::*;
pub use crate::common::parse::*;
pub use crate::common::scrape::*;
pub use crate::common::sibling::*;

mod methods;
mod parse;
mod sibling;
mod scrape;

pub const BASE_TAGS_URL: &str = "https://www.w3schools.com/tags/";

#[cfg(feature = "build")]
pub const BASE_JS_URL: &str = "https://www.w3schools.com/jsref/";

const OFFLINE_PATH: &str = "offline";

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UrlOption {
    //
    Optional,

    //
    Required,
}
