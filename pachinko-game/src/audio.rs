//! Procedural audio synthesis. No external sample files.
//!
//! Generates WAV byte buffers in-memory for chucker chime, BGM loops,
//! fanfare brass, bust SFX, etc. Plays back via macroquad::audio.
//!
//! Per PRD R-14..R-16: BGM transitions, chucker chime latency,
//! fanfare uninterruptibility are all driven from here.

use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams, Sound};

const SAMPLE_RATE: u32 = 22_050;

pub struct AudioBank {
    pub chucker_chime: Sound,
    pub bust_sfx: Sound,
    pub hit_fanfare: Sound,
    pub base_bgm: Sound,
    pub reach_bgm: Sound,
    pub kakuhen_bgm: Sound,
    pub confirmed_cue: Sound,
    pub jackpot_fanfare: Sound,
}

impl AudioBank {
    pub async fn build() -> Self {
        Self {
            chucker_chime: load(&wav(&synth_chime())).await,
            bust_sfx: load(&wav(&synth_bust())).await,
            hit_fanfare: load(&wav(&synth_hit_fanfare())).await,
            base_bgm: load(&wav(&synth_base_bgm())).await,
            reach_bgm: load(&wav(&synth_reach_bgm())).await,
            kakuhen_bgm: load(&wav(&synth_kakuhen_bgm())).await,
            confirmed_cue: load(&wav(&synth_confirmed_cue())).await,
            jackpot_fanfare: load(&wav(&synth_jackpot_fanfare())).await,
        }
    }
}

async fn load(bytes: &[u8]) -> Sound {
    load_sound_from_bytes(bytes).await.expect("synthesized WAV must load")
}

pub fn play_one(sound: &Sound, volume: f32) {
    play_sound(sound, PlaySoundParams { looped: false, volume });
}

pub fn play_loop(sound: &Sound, volume: f32) {
    play_sound(sound, PlaySoundParams { looped: true, volume });
}

pub fn stop(sound: &Sound) {
    stop_sound(sound);
}

// ---------- WAV encoder ----------

fn wav(samples: &[i16]) -> Vec<u8> {
    let n = samples.len() as u32;
    let byte_rate = SAMPLE_RATE * 2;
    let data_size = n * 2;
    let mut out = Vec::with_capacity(44 + data_size as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_size).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&1u16.to_le_bytes()); // mono
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes()); // block align
    out.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_size.to_le_bytes());
    for s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

// ---------- Oscillator helpers ----------

fn sine(t: f32, freq: f32) -> f32 {
    (t * freq * std::f32::consts::TAU).sin()
}

fn saw(t: f32, freq: f32) -> f32 {
    let p = (t * freq).fract();
    2.0 * p - 1.0
}

fn square(t: f32, freq: f32, duty: f32) -> f32 {
    let p = (t * freq).fract();
    if p < duty { 1.0 } else { -1.0 }
}

/// ADSR-ish envelope: attack-decay-sustain-release.
fn env(t: f32, a: f32, d: f32, s_level: f32, s_time: f32, r: f32) -> f32 {
    if t < a { t / a.max(0.0001) }
    else if t < a + d {
        let x = (t - a) / d.max(0.0001);
        1.0 - x * (1.0 - s_level)
    }
    else if t < a + d + s_time { s_level }
    else if t < a + d + s_time + r {
        let x = (t - a - d - s_time) / r.max(0.0001);
        s_level * (1.0 - x)
    } else { 0.0 }
}

fn to_i16(x: f32) -> i16 {
    (x.clamp(-1.0, 1.0) * 32_000.0) as i16
}

// ---------- Patches ----------

/// Bright bell ping. ~250ms.
fn synth_chime() -> Vec<i16> {
    let dur = 0.30;
    let n = (dur * SAMPLE_RATE as f32) as usize;
    (0..n).map(|i| {
        let t = i as f32 / SAMPLE_RATE as f32;
        let e = env(t, 0.005, 0.05, 0.5, 0.05, 0.15);
        let s = sine(t, 1760.0) * 0.6 + sine(t, 2200.0) * 0.3 + sine(t, 3300.0) * 0.15;
        to_i16(s * e * 0.6)
    }).collect()
}

/// Bust SFX: descending sine. ~600ms.
fn synth_bust() -> Vec<i16> {
    let dur = 0.6;
    let n = (dur * SAMPLE_RATE as f32) as usize;
    (0..n).map(|i| {
        let t = i as f32 / SAMPLE_RATE as f32;
        let freq = 600.0 * (1.0 - t / dur).max(0.05);
        let e = env(t, 0.02, 0.1, 0.4, 0.3, 0.18);
        let s = sine(t, freq) * 0.6 + saw(t, freq * 0.5) * 0.2;
        to_i16(s * e * 0.4)
    }).collect()
}

/// Brass hit fanfare on reach hit. ~1.5s. Major chord, brass-ish saws.
fn synth_hit_fanfare() -> Vec<i16> {
    let dur = 1.5;
    let n = (dur * SAMPLE_RATE as f32) as usize;
    let chord = [261.63, 329.63, 392.00, 523.25]; // C-E-G-C (C major)
    (0..n).map(|i| {
        let t = i as f32 / SAMPLE_RATE as f32;
        let e = env(t, 0.02, 0.15, 0.7, 1.0, 0.3);
        let mut s = 0.0;
        for f in &chord {
            s += saw(t, *f) * 0.18 + square(t, *f * 0.5, 0.4) * 0.06;
        }
        // Cymbal-ish noise burst at attack
        let noise = if t < 0.08 { ((t * 99_991.0).sin() * 1000.0).sin() * (1.0 - t / 0.08) * 0.4 } else { 0.0 };
        to_i16((s * e * 0.5 + noise * 0.3) * 0.8)
    }).collect()
}

/// Base BGM loop: minor key marimba ostinato. ~4s loop.
fn synth_base_bgm() -> Vec<i16> {
    let dur = 4.0;
    let n = (dur * SAMPLE_RATE as f32) as usize;
    // A minor pentatonic ostinato: A2 C3 E3 D3 A2 C3 E3 G3
    let notes = [110.0, 130.81, 164.81, 146.83, 110.0, 130.81, 164.81, 196.00];
    let step_dur = dur / notes.len() as f32;
    (0..n).map(|i| {
        let t = i as f32 / SAMPLE_RATE as f32;
        let step = ((t / step_dur) as usize) % notes.len();
        let local = (t / step_dur).fract() * step_dur;
        let e = env(local, 0.005, 0.1, 0.3, step_dur * 0.5, step_dur * 0.4);
        let f = notes[step];
        let s = sine(t, f) * 0.5 + sine(t, f * 2.0) * 0.15;
        to_i16(s * e * 0.2)
    }).collect()
}

/// Reach BGM: tense, descending bass line + tremolo. ~3s loop.
fn synth_reach_bgm() -> Vec<i16> {
    let dur = 3.0;
    let n = (dur * SAMPLE_RATE as f32) as usize;
    (0..n).map(|i| {
        let t = i as f32 / SAMPLE_RATE as f32;
        let tremolo = 0.7 + 0.3 * sine(t, 6.0);
        let bass = saw(t, 82.4) * 0.3; // E2
        let mid = saw(t, 165.0) * 0.2;
        let hi = square(t, 329.6, 0.5) * 0.1;
        to_i16((bass + mid + hi) * tremolo * 0.35)
    }).collect()
}

/// Kakuhen BGM: bright major upbeat. ~3s loop.
fn synth_kakuhen_bgm() -> Vec<i16> {
    let dur = 3.0;
    let n = (dur * SAMPLE_RATE as f32) as usize;
    let notes = [261.6, 329.6, 392.0, 329.6, 440.0, 392.0, 329.6, 261.6];
    let step_dur = dur / notes.len() as f32;
    (0..n).map(|i| {
        let t = i as f32 / SAMPLE_RATE as f32;
        let step = ((t / step_dur) as usize) % notes.len();
        let local = (t / step_dur).fract() * step_dur;
        let e = env(local, 0.008, 0.05, 0.5, step_dur * 0.4, step_dur * 0.4);
        let f = notes[step];
        let s = saw(t, f) * 0.25 + saw(t, f * 0.5) * 0.18 + square(t, f * 2.0, 0.3) * 0.08;
        to_i16(s * e * 0.3)
    }).collect()
}

/// Confirmed-reach cue: dramatic opening-theme stinger. ~2s.
fn synth_confirmed_cue() -> Vec<i16> {
    let dur = 2.0;
    let n = (dur * SAMPLE_RATE as f32) as usize;
    // A minor → F major → C major → E major progression (4 chords, 0.5s each)
    let chords: [[f32; 3]; 4] = [
        [220.0, 261.6, 329.6], // Am
        [174.6, 220.0, 261.6], // F
        [196.0, 246.9, 329.6], // C/G
        [164.8, 207.7, 246.9], // E
    ];
    let chord_dur = dur / chords.len() as f32;
    (0..n).map(|i| {
        let t = i as f32 / SAMPLE_RATE as f32;
        let idx = ((t / chord_dur) as usize).min(chords.len() - 1);
        let local = t - idx as f32 * chord_dur;
        let e = env(local, 0.02, 0.1, 0.7, chord_dur * 0.5, chord_dur * 0.3);
        let chord = chords[idx];
        let mut s = 0.0;
        for f in &chord {
            s += saw(t, *f) * 0.18 + square(t, *f * 2.0, 0.4) * 0.06;
        }
        to_i16(s * e * 0.5)
    }).collect()
}

/// Jackpot fanfare: long brass-heavy major-key catharsis. ~6s.
fn synth_jackpot_fanfare() -> Vec<i16> {
    let dur = 6.0;
    let n = (dur * SAMPLE_RATE as f32) as usize;
    // Two phrases: stinger (1s), sustain + arpeggio (5s)
    (0..n).map(|i| {
        let t = i as f32 / SAMPLE_RATE as f32;
        let s = if t < 1.0 {
            let chord = [261.6, 329.6, 392.0, 523.3];
            let e = env(t, 0.02, 0.2, 0.7, 0.7, 0.1);
            let mut out = 0.0;
            for f in &chord {
                out += saw(t, *f) * 0.15 + square(t, *f * 0.5, 0.5) * 0.08;
            }
            let noise = if t < 0.1 { ((t * 97_999.0).sin() * 1000.0).sin() * (1.0 - t / 0.1) * 0.6 } else { 0.0 };
            out * e + noise * 0.4
        } else {
            let phase = t - 1.0;
            let arp_notes = [261.6, 329.6, 392.0, 523.3, 392.0, 329.6];
            let arp_step = 0.1;
            let step = ((phase / arp_step) as usize) % arp_notes.len();
            let local = (phase / arp_step).fract() * arp_step;
            let e_arp = env(local, 0.005, 0.03, 0.4, arp_step * 0.3, arp_step * 0.3);
            let arp = saw(t, arp_notes[step]) * 0.18 * e_arp;
            // Sustained bass + brass
            let bass = saw(t, 130.8) * 0.18 + saw(t, 196.0) * 0.12;
            let global_e = env(phase, 0.05, 0.2, 0.7, 4.0, 0.5);
            (arp + bass) * global_e
        };
        to_i16(s * 0.55)
    }).collect()
}
