use anyhow::{anyhow, Context, Result};
use std::{ffi::OsStr, path::Path};

pub fn get_froms(path: &str) -> Result<Vec<String>> {
    let path = Path::new(path);
    let xattr_name = OsStr::new("com.apple.metadata:kMDItemWhereFroms");
    let val_opt = xattr::get(path, &xattr_name).with_context(|| {
        format!(
            "Could not get xattr for {}",
            path.to_str().unwrap_or("(non-UTF-8 path)")
        )
    })?;

    if let Some(attr_val) = val_opt {
        let parsed: Vec<String> = plist::from_bytes(&attr_val).with_context(|| {
            format!(
                "Could not get kMDItemWhereFroms attribute for file {}",
                path.to_str().unwrap_or("(non-UTF-8 path)")
            )
        })?;
        Ok(parsed)
    } else {
        Err(anyhow!(
            "Missing xattr value for kMDItemWhereFroms for file {}",
            path.to_str().unwrap_or("(non-UTF-8 path)")
        ))
    }
}
