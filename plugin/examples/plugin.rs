#![no_main]

use extism_pdk::*;
use helix_plugin::*;

#[plugin_fn]
pub fn vsplit_copy_sel() -> FnResult<()> {
    for sel in Editor.selections() {
        Editor.execute(":vsplit-new")?;
        Editor.add_selection(0, 0);
        let txt = sel.text()?;
        Editor.insert_text(&txt, Insert::BeforeSelection)?;
    }
    Ok(())
}

#[plugin_fn]
pub fn open_sel() -> FnResult<()> {
    for sel in Editor.selections() {
        let txt = sel.text()?;
        Editor.execute(":vsplit-new")?;
        Editor.open(&txt)?;
    }
    Ok(())
}
