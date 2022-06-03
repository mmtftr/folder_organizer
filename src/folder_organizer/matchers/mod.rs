use anyhow::{anyhow, Result};
use serde_json::Value;

use super::util::matcher::Matcher;

mod eval_js;
mod hash_storage;
mod user_selection;

pub fn get_matcher_from_name(
    matcher_name: impl AsRef<str>,
    options: &Option<Value>,
) -> Result<Box<dyn Matcher>> {
    let matcher_name = matcher_name.as_ref();

    match (matcher_name, options) {
        (eval_js::MATCHER_NAME, Some(options)) => Ok(Box::new(
            eval_js::EvalJsDecider::try_new_with_opts(options)?,
        )),
        (user_selection::MATCHER_NAME, _) => {
            Ok(Box::new(user_selection::UserSelectionMatcher::new()))
        }
        (hash_storage::MATCHER_NAME, _) => Ok(Box::new(hash_storage::HashStorageMatcher::new())),
        _ => Err(anyhow!(
            "Could not find suitable matcher by the name {}",
            matcher_name
        )),
    }
}
