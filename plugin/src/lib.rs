mod bindings;

extern crate extism_pdk;

#[derive(Clone, Copy, Debug)]
pub struct Editor;

#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Selection(u64);

impl Selection {
    pub fn from(self) -> usize {
        unsafe { bindings::selection_begin(self.0) as usize }
    }

    pub fn to(self) -> usize {
        unsafe { bindings::selection_end(self.0) as usize }
    }

    pub fn text(&self) -> Result<String, extism_pdk::Error> {
        let from = self.from();
        let to = self.to();
        let ptr = unsafe { bindings::text(from as u64, to as u64) };
        let ptr = extism_pdk::Memory::find(ptr).unwrap();
        let res = ptr.to();
        ptr.free();
        res
    }
}

impl Editor {
    pub fn add_selection(self, start: usize, end: usize) -> Selection {
        let n = unsafe { bindings::selection_add(start as u64, end as u64) };
        Selection(n)
    }

    pub fn selections(self) -> impl Iterator<Item = Selection> {
        let len = unsafe { bindings::selection_count() };

        let mut n = 0;
        std::iter::from_fn(move || {
            if n < len {
                let x = Selection(n);
                n += 1;
                return Some(x);
            }

            None
        })
    }

    pub fn save<P: AsRef<std::path::Path>>(
        self,
        filename: Option<P>,
    ) -> Result<(), extism_pdk::Error> {
        let ptr = if let Some(filename) = filename {
            extism_pdk::Memory::new(&filename.as_ref().to_str().unwrap_or_default())?
        } else {
            extism_pdk::Memory::null()
        };
        unsafe { bindings::save(ptr.offset()) };
        ptr.free();
        Ok(())
    }

    pub fn set_path<P: AsRef<std::path::Path>>(self, filename: P) -> Result<(), extism_pdk::Error> {
        let ptr = extism_pdk::Memory::new(&filename.as_ref().to_str().unwrap_or_default())?;
        unsafe { bindings::set_path(ptr.offset()) }
        ptr.free();
        Ok(())
    }

    pub fn open<P: AsRef<std::path::Path>>(self, filename: P) -> Result<(), extism_pdk::Error> {
        let ptr = extism_pdk::Memory::new(&filename.as_ref().to_str().unwrap_or_default())?;
        unsafe { bindings::open(ptr.offset()) }
        ptr.free();
        Ok(())
    }

    pub fn close(self) {
        unsafe { bindings::close() }
    }

    pub fn undo(self) {
        unsafe { bindings::undo() }
    }

    pub fn redo(self) {
        unsafe { bindings::redo() }
    }

    pub fn focus_next(self) {
        unsafe { bindings::focus_next() }
    }

    pub fn focus_prev(self) {
        unsafe { bindings::focus_prev() }
    }

    pub fn hsplit(self) {
        unsafe { bindings::hsplit() }
    }

    pub fn vsplit(self) {
        unsafe { bindings::vsplit() }
    }

    pub fn clear_selection(self) {
        unsafe { bindings::selection_reset() }
    }

    pub fn insert_text(
        self,
        text: impl AsRef<str>,
        insert: Insert,
    ) -> Result<(), extism_pdk::Error> {
        let ptr = extism_pdk::Memory::new(&text.as_ref())?;
        match insert {
            Insert::BeforeSelection => unsafe {
                bindings::selection_insert_text_before(ptr.offset())
            },
            Insert::AfterSelection => unsafe {
                bindings::selection_insert_text_after(ptr.offset())
            },
        }
        ptr.free();
        Ok(())
    }

    pub fn replace_text(&self, txt: impl AsRef<str>) -> Result<(), extism_pdk::Error> {
        let ptr = extism_pdk::Memory::new(&txt.as_ref())?;
        unsafe { bindings::selection_replace_text(ptr.offset()) };
        ptr.free();
        Ok(())
    }

    pub fn set_status(self, text: impl AsRef<str>) -> Result<(), extism_pdk::Error> {
        let ptr = extism_pdk::Memory::new(&text.as_ref())?;
        unsafe {
            bindings::set_status(ptr.offset());
        }
        ptr.free();
        Ok(())
    }

    pub fn clear_status(self) {
        unsafe {
            bindings::clear_status();
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Insert {
    BeforeSelection,
    AfterSelection,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
