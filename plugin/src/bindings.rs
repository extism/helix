
// Generated with gen.py

pub type ExtismPointer = u64;

#[link(wasm_import_module = "helix:editor/env")]
extern "C" {
  pub fn save(ExtismPointer) -> ;
  pub fn set_status(ExtismPointer) -> ;
  pub fn clear_status() -> ;
  pub fn set_path(ExtismPointer) -> ;
  pub fn undo() -> ;
  pub fn redo() -> ;
  pub fn open(ExtismPointer) -> ;
  pub fn close() -> ;
  pub fn vsplit() -> ;
  pub fn hsplit() -> ;
  pub fn focus_next() -> ;
  pub fn focus_prev() -> ;
  pub fn selection_insert_text_after(ExtismPointer) -> ;
  pub fn selection_insert_text_before(ExtismPointer) -> ;
  pub fn selection_replace_text(ExtismPointer) -> ;
  pub fn selection_add(u64, u64) -> u64;
  pub fn selection_reset() -> ;
  pub fn selection_count() -> u64;
  pub fn selection_begin(u64) -> u64;
  pub fn selection_end(u64) -> u64;
  pub fn text(u64, u64) -> ExtismPointer;
}
