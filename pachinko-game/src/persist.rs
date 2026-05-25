//! Save/load wrapper. Native: writes to local file under data dir.
//! WASM: stores in localStorage via macroquad::file? Macroquad has no localStorage —
//! we use sapp_jsutils on web for that. For MVP, use simple persistent file on native
//! and an in-memory placeholder on WASM (with a TODO for localStorage).

use pachinko_core::save::{deserialize, serialize, SaveData};

#[cfg(not(target_arch = "wasm32"))]
const SAVE_PATH: &str = "pachinko_save.json";

#[cfg(not(target_arch = "wasm32"))]
pub fn save(data: &SaveData) {
    if let Ok(s) = serialize(data) {
        let _ = std::fs::write(SAVE_PATH, s);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load() -> Option<SaveData> {
    let s = std::fs::read_to_string(SAVE_PATH).ok()?;
    deserialize(&s).ok()
}

// ---- WASM path ----
// Macroquad does not expose localStorage directly; we use a thin JS shim via
// the browser's window.localStorage. Until shim is wired, no-op.

#[cfg(target_arch = "wasm32")]
pub fn save(_data: &SaveData) {
    // TODO: wire localStorage via JS shim. MVP: no-op.
}

#[cfg(target_arch = "wasm32")]
pub fn load() -> Option<SaveData> {
    None
}
