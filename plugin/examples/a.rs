#![no_main]

use extism_pdk::*;
use helix_plugin::*;

const EXTENSIONS: &'static [(&'static str, &'static str)] =
    &[("c", "h"), ("cpp", "hpp"), ("cc", "hh"), ("ml", "mli")];

fn switch_header_impl<F: Fn() -> Result<(), Error>>(f: F) -> FnResult<()> {
    if let Some(path) = Editor.path()? {
        if let Some(ext) = path.extension() {
            for (k, v) in EXTENSIONS {
                if &ext == k {
                    f()?;
                    Editor.open(path.with_extension(v))?;
                } else if &ext == v {
                    f()?;
                    Editor.open(path.with_extension(k))?;
                }
            }
        }
    } else {
        Editor.set_status("Unable to determine file name")?;
    }

    Ok(())
}

#[plugin_fn]
pub fn switch_header() -> FnResult<()> {
    switch_header_impl(|| Ok(()))
}

#[plugin_fn]
pub fn vsplit_header() -> FnResult<()> {
    switch_header_impl(|| Editor.execute(":vsplit-new"))
}

#[plugin_fn]
pub fn hsplit_header() -> FnResult<()> {
    switch_header_impl(|| Editor.execute(":hsplit-new"))
}

#[plugin_fn]
pub fn vsplit_sel() -> FnResult<()> {
    for sel in Editor.selections() {
        let txt = sel.text()?;
        Editor.execute(":vsplit-new")?;
        Editor.open(&txt)?;
    }
    Ok(())
}

#[plugin_fn]
pub fn hsplit_sel() -> FnResult<()> {
    for sel in Editor.selections() {
        let txt = sel.text()?;
        Editor.execute(":hsplit-new")?;
        Editor.open(&txt)?;
    }
    Ok(())
}

#[plugin_fn]
pub fn open_sel() -> FnResult<()> {
    for sel in Editor.selections() {
        let txt = sel.text()?;
        Editor.open(&txt)?;
    }
    Ok(())
}
