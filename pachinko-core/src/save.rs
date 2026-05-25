//! Save/load with versioning + checksum. Per PRD R-18..R-20.
//! Pulls in std-only deps; gated behind the `std` feature.

use serde::{Deserialize, Serialize};

use crate::coordinator::CabinetCoordinator;
use crate::state::GameState;

pub const SAVE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub schema_version: u32,
    pub state: GameState,
    pub coord: CabinetCoordinator,
    pub rng_seed: u64,
    pub rng_position: u64, // logical: total spins consumed
    /// FNV-1a checksum over (schema_version || serialized rest). Set by `seal`.
    pub checksum: u64,
}

impl SaveData {
    pub fn new(state: GameState, coord: CabinetCoordinator, seed: u64) -> Self {
        let mut s = Self {
            schema_version: SAVE_SCHEMA_VERSION,
            state,
            coord,
            rng_seed: seed,
            rng_position: 0,
            checksum: 0,
        };
        s.checksum = s.compute_checksum();
        s
    }

    fn compute_checksum(&self) -> u64 {
        let body = SaveBody {
            schema_version: self.schema_version,
            state: self.state,
            coord: self.coord.clone(),
            rng_seed: self.rng_seed,
            rng_position: self.rng_position,
        };
        let bytes = serde_json::to_vec(&body).unwrap_or_default();
        fnv1a64(&bytes)
    }

    pub fn validate(&self) -> Result<(), SaveError> {
        if self.schema_version != SAVE_SCHEMA_VERSION {
            return Err(SaveError::VersionMismatch {
                expected: SAVE_SCHEMA_VERSION,
                got: self.schema_version,
            });
        }
        let expected = self.compute_checksum();
        if expected != self.checksum {
            return Err(SaveError::ChecksumMismatch);
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SaveBody {
    schema_version: u32,
    state: GameState,
    coord: CabinetCoordinator,
    rng_seed: u64,
    rng_position: u64,
}

#[derive(Debug, PartialEq)]
pub enum SaveError {
    VersionMismatch { expected: u32, got: u32 },
    ChecksumMismatch,
    Parse(String),
}

impl core::fmt::Display for SaveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SaveError::VersionMismatch { expected, got } =>
                write!(f, "save schema version mismatch: expected {expected}, got {got}"),
            SaveError::ChecksumMismatch => write!(f, "save checksum mismatch"),
            SaveError::Parse(s) => write!(f, "save parse error: {s}"),
        }
    }
}

impl std::error::Error for SaveError {}

/// FNV-1a 64-bit hash. Tiny, deterministic, no_std-friendly.
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

pub fn serialize(save: &SaveData) -> Result<String, SaveError> {
    serde_json::to_string(save).map_err(|e| SaveError::Parse(e.to_string()))
}

pub fn deserialize(s: &str) -> Result<SaveData, SaveError> {
    let save: SaveData = serde_json::from_str(s).map_err(|e| SaveError::Parse(e.to_string()))?;
    save.validate()?;
    Ok(save)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// PRD R-19: round-trip works; checksum validates.
    #[test]
    fn save_roundtrip() {
        let state = GameState::new_session(123);
        let coord = CabinetCoordinator::new();
        let save = SaveData::new(state, coord, 0xDEAD_BEEF);
        let s = serialize(&save).expect("serialize");
        let loaded = deserialize(&s).expect("deserialize");
        assert_eq!(loaded.rng_seed, 0xDEAD_BEEF);
        assert_eq!(loaded.state.total_spins, 0);
    }

    /// PRD R-19: corrupt save detected.
    #[test]
    fn save_corruption_detected() {
        let state = GameState::new_session(123);
        let coord = CabinetCoordinator::new();
        let save = SaveData::new(state, coord, 0xDEAD_BEEF);
        let mut s = serialize(&save).expect("serialize");
        // Corrupt: change a digit in rng_seed
        s = s.replace("\"rng_seed\":3735928559", "\"rng_seed\":3735928560");
        let err = deserialize(&s).expect_err("must reject");
        assert_eq!(err, SaveError::ChecksumMismatch);
    }
}
