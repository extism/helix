#![no_main]

use extism_pdk::*;
use helix_plugin::*;

#[plugin_fn]
pub fn vsplit_sel() -> FnResult<()> {
    let txt = Selection::default().text()?;
    Editor.execute(":vsplit-new")?;
    Editor.add_selection(0, 0);
    Editor.insert_text(&txt, Insert::BeforeSelection)?;
    Editor.select_all()?;
    Ok(())
}

#[plugin_fn]
pub fn open_sel() -> FnResult<()> {
    for sel in Editor.selections() {
        let txt = sel.text()?;
        Editor.execute(":vsplit-new")?;
        Editor.open(&txt)?;
        Editor.focus_prev();
    }
    Ok(())
}
