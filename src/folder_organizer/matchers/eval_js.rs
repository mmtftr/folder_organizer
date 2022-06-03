use crate::folder_organizer::config::OrganizerConfig;

use super::super::util::{file::FileInfo, file_action::FileAction, matcher::Matcher};
use anyhow::{anyhow, Result};
use quick_js::{Context, JsValue};
use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub struct EvalJsDecider {
    js_loc: String,
}

const FUNCTION_NAME: &str = "get_actions";
impl EvalJsDecider {
    pub fn try_new_with_opts(options: &Value) -> Result<Self> {
        match options {
            Value::Object(obj) => {
                let js_loc = obj.get("js_loc").unwrap().as_str().unwrap().to_string();
                Ok(EvalJsDecider { js_loc })
            }
            _ => Err(anyhow!(
                "EvalJsDecider::new_with_opts: options must be an object"
            )),
        }
    }

    pub fn eval_js_for_file<'a>(
        &self,
        file_info: &FileInfo,
        _: &OrganizerConfig,
    ) -> Option<FileAction> {
        let mut file = File::open(&self.js_loc).unwrap();
        let mut js_code = String::new();

        if let Err(_) = file.read_to_string(&mut js_code) {
            log::error!("Could not read JS file");
            return None;
        }

        let js_ctx = Context::new();
        if let Err(ctx_err) = js_ctx {
            log::error!("Could not create JS context. Error {:?}", ctx_err);
            return None;
        }

        let js_ctx = js_ctx.unwrap();
        if let Err(js_err) = js_ctx.eval(&js_code) {
            log::error!("Could not evaluate JS code. Error {:?}", js_err);
            return None;
        }

        js_ctx
            .eval("var _JSON_parse = JSON.parse, _JSON_stringify = JSON.stringify;")
            .unwrap();
        // TODO: this is *VERY VERY* inefficient. Ideally we'd be directly
        // converting to a JsValue but this needs to be manually written or be done via serde.
        let file_info = serde_json::to_string(file_info).ok()?;
        let file_info = js_ctx
            .call_function("_JSON_parse", [JsValue::String(file_info)])
            .ok()?;

        let action = js_ctx.call_function(FUNCTION_NAME, [file_info]);
        if let Err(action_err) = action {
            log::error!("Could not get action from JS. Error {:?}", action_err);
            return None;
        }
        let action = action.ok()?;

        // quick_js::JsValue is not Serialize, so we convert to JSON
        // string and deserialize a FileAction from that string.
        let json_str = js_ctx.call_function("_JSON_stringify", [action]).ok()?;

        // TODO: write a custom deserializer or a serde deserializer
        // that can handle JsValue
        let deser = serde_json::from_str(json_str.as_str()?);
        deser.ok()
    }
}

impl Matcher for EvalJsDecider {
    fn match_action(&self, file_info: &FileInfo, config: &OrganizerConfig) -> FileAction {
        self.eval_js_for_file(file_info, config)
            .unwrap_or(FileAction::Fallthrough)
    }
}

pub const MATCHER_NAME: &str = "eval_js";

#[cfg(test)]
mod tests {}
