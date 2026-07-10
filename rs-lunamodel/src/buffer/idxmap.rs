use std::ffi::CStr;

use indexmap::IndexMap;
use lunamodel_cext_vtable::LmIndexMapItemBuffer;

use crate::{Column, Result, api::api, traits::LmTryInto};

impl LmTryInto<IndexMap<String, Column>> for LmIndexMapItemBuffer {
    fn lm_try_into(self) -> Result<IndexMap<String, Column>> {
        let mut map = IndexMap::with_capacity(self.len);

        if !self.data.is_null() {
            // The buffer owns each key string and column handle, and
            // `index_map_buffer_free` frees both. We *copy* the keys (leaving the
            // originals for the free), but *take* the column handles into owning
            // `Column`s — so we null each `value` slot to stop the free from
            // dropping a handle we now own (that would be a double-free).
            let items = unsafe { std::slice::from_raw_parts_mut(self.data, self.len) };
            for item in items {
                let key = if item.key.is_null() {
                    String::new()
                } else {
                    unsafe { CStr::from_ptr(item.key) }
                        .to_string_lossy()
                        .into_owned()
                };

                let column = Column::from_raw(item.value);
                item.value = std::ptr::null_mut();

                map.insert(key, column);
            }
        }

        // Frees the item array + the (still-owned) key strings; skips the
        // nulled-out column handles we moved into `map`.
        unsafe { (api().index_map_buffer_free)(self) };

        Ok(map)
    }
}
