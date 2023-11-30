pub type ExtismPointer = u64;
pub type HelixSelection = u64;
pub type HelixPosition = u64;

#[link(wasm_import_module = "helix:editor/env")]
extern "C" {
    pub fn set_status(arg1: ExtismPointer);
    pub fn clear_status();
    pub fn set_path(arg1: ExtismPointer);
    pub fn text(arg1: HelixPosition, arg2: HelixPosition) -> ExtismPointer;
    pub fn selection_insert_text_before(arg1: ExtismPointer);
    pub fn selection_insert_text_after(arg1: ExtismPointer);
    pub fn selection_replace_text(arg1: ExtismPointer);
    pub fn selection_add(arg1: HelixPosition, arg2: HelixPosition) -> u64;
    pub fn selection_reset();
    pub fn selection_begin(arg1: HelixSelection) -> HelixPosition;
    pub fn selection_end(arg1: HelixSelection) -> HelixPosition;
    pub fn selection_count() -> u64;
    pub fn vsplit();
    pub fn hsplit();
    pub fn focus_next();
    pub fn focus_prev();
    pub fn close();
    pub fn undo();
    pub fn redo();
    pub fn save(arg1: ExtismPointer);
    pub fn open(arg1: ExtismPointer);
}
