#![no_main]

use extism_pdk::*;
use helix_plugin::*;

const EXTENSIONS: &'static [(&'static str, &'static str)] =
    &[("c", "h"), ("cpp", "hpp"), ("cc", "hh"), ("ml", "mli")];

#[plugin_fn]
pub fn open_header() -> FnResult<()> {
    if let Some(path) = Editor.path()? {
        if let Some(ext) = path.extension() {
            for (k, v) in EXTENSIONS {
                if &ext == k {
                    Editor.open(path.with_extension(v))?;
                } else if &ext == v {
                    Editor.open(path.with_extension(k))?;
                }
            }
        }
    } else {
        Editor.set_status("Unable to determine file name")?;
    }

    Ok(())
}
