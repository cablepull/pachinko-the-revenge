//! Iter-4 persistence layer. Per PRD-004 R-54 + ADR-002.
//!
//! Schema (versioned): `pachinko-the-revenge:v1`. Stores:
//! - PinLayout knob values (story 001)
//! - Unlocked chapter (story 005)
//! - Last session summary (story 006)
//! - Timestamp of last session (gates the welcome-back card per R-53)
//!
//! WASM: quad-storage (wraps localStorage). Native: JSON file under cwd.
//! On any failure (corrupt JSON, version mismatch), fall back to fresh
//! session per anti-pattern "persistence must not gate gameplay."

use serde::{Deserialize, Serialize};

use crate::playfield::PinLayout;

const STORAGE_KEY: &str = "pachinko-the-revenge:v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionSummary {
    pub duration_ms: u64,
    pub balls_fired: u32,
    pub balls_won: u64,
    pub net_yen: i64,
    pub highest_chapter: u32,
    pub longest_dry_streak: u32,
    pub rarest_reach_tier: Option<String>, // "calm"|"mid"|"premium"|"confirmed"
    pub narrative_lines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedState {
    pub schema_version: u32,
    pub layout: PinLayout,
    pub unlocked_chapter: u32,
    pub last_summary: Option<SessionSummary>,
    pub last_session_at_ms: u64,
}

impl Default for PersistedState {
    fn default() -> Self {
        Self {
            schema_version: 1,
            layout: PinLayout::default(),
            unlocked_chapter: 1,
            last_summary: None,
            last_session_at_ms: 0,
        }
    }
}

// ---- Native path ----

#[cfg(not(target_arch = "wasm32"))]
const SAVE_PATH: &str = "pachinko_save_v1.json";

#[cfg(not(target_arch = "wasm32"))]
pub fn save(state: &PersistedState) {
    if let Ok(s) = serde_json::to_string(state) {
        let _ = std::fs::write(SAVE_PATH, s);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load() -> Option<PersistedState> {
    let s = std::fs::read_to_string(SAVE_PATH).ok()?;
    let st: PersistedState = serde_json::from_str(&s).ok()?;
    if st.schema_version != 1 { return None; }
    Some(st)
}

// ---- WASM path (quad-storage) ----

#[cfg(target_arch = "wasm32")]
pub fn save(state: &PersistedState) {
    if let Ok(s) = serde_json::to_string(state) {
        let mut storage = quad_storage::STORAGE.lock().unwrap();
        storage.set(STORAGE_KEY, &s);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn load() -> Option<PersistedState> {
    let storage = quad_storage::STORAGE.lock().ok()?;
    let s = storage.get(STORAGE_KEY)?;
    let st: PersistedState = serde_json::from_str(&s).ok()?;
    if st.schema_version != 1 { return None; }
    Some(st)
}

// ---- Tests ----

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playfield::Knob;

    /// Per Story 005: round-trip native.
    #[test]
    fn persisted_state_roundtrip() {
        let mut layout = PinLayout::stock();
        layout.set(Knob::LeftFunnelTilt, 0.3);
        layout.set(Knob::ChuckerMouthWidth, -0.5);
        let s = PersistedState {
            schema_version: 1,
            layout,
            unlocked_chapter: 3,
            last_summary: Some(SessionSummary {
                duration_ms: 600_000,
                balls_fired: 1200,
                balls_won: 1440,
                net_yen: 960,
                highest_chapter: 3,
                longest_dry_streak: 287,
                rarest_reach_tier: Some("premium".into()),
                narrative_lines: vec!["You survived an 847-spin hama-dai.".into()],
            }),
            last_session_at_ms: 1_700_000_000_000,
        };
        let json = serde_json::to_string(&s).unwrap();
        let loaded: PersistedState = serde_json::from_str(&json).unwrap();
        assert_eq!(loaded.layout.get(Knob::LeftFunnelTilt), 0.3);
        assert_eq!(loaded.layout.get(Knob::ChuckerMouthWidth), -0.5);
        assert_eq!(loaded.unlocked_chapter, 3);
        assert!(loaded.last_summary.is_some());
    }

    /// Per Story 005: corrupt JSON falls back to None (no panic).
    #[test]
    fn persisted_state_corrupt_returns_none() {
        let invalid = "{not valid json";
        let res: Result<PersistedState, _> = serde_json::from_str(invalid);
        assert!(res.is_err());
    }

    /// Per Story 005: wrong schema version returns None.
    #[test]
    fn persisted_state_version_mismatch() {
        let bad = r#"{"schema_version":99,"layout":{"knobs":[0,0,0,0,0,0]},"unlocked_chapter":1,"last_summary":null,"last_session_at_ms":0}"#;
        let st: Option<PersistedState> = serde_json::from_str::<PersistedState>(bad)
            .ok()
            .filter(|s| s.schema_version == 1);
        assert!(st.is_none());
    }
}
