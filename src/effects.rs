use serde::Serialize;
use std::collections::HashMap;

pub trait Effect {
    /// `bins` is a *cumulative* array:
    /// bins[i] = sum_{k=0..i} original_bins[k]
    fn apply(&self, bins: &[f32], dmx: &mut HashMap<u16, u8>);
}

/// Helper: sum of a slice using cumulative bins.
/// If bins = prefix-sum array, then:
///   sum(a..=b) = bins[b] - bins[a-1]
#[inline]
fn cum_sum(bins: &[f32], a: usize, b: usize) -> f32 {
    if bins.is_empty() || a > b || b >= bins.len() {
        return 0.0;
    }
    if a == 0 {
        bins[b]
    } else {
        bins[b] - bins[a - 1]
    }
}

//
// ──────────────────────────────────────────────────────────────
// LINK EFFECT  (sum → [lower, upper] → DMX)
// bins = vector of (start,end) pairs represented implicitly
// with cumulative indexing
// ──────────────────────────────────────────────────────────────
//

pub struct LinkEffect {
    pub dmx_channels: Vec<u16>,
    pub bins: Vec<(usize, usize)>,   // cumulative slice ranges
    pub lower: f32,
    pub upper: f32,
}

impl LinkEffect {
    pub fn new(dmx_channels: Vec<u16>, bins: Vec<(usize, usize)>) -> Self {
        Self {
            dmx_channels,
            bins,
            lower: 0.0,
            upper: 1.0,
        }
    }
}

impl Effect for LinkEffect {
    fn apply(&self, bins: &[f32], dmx: &mut HashMap<u16, u8>) {

        // SUM SLICES USING PREFIX-SUM
        let mut sum = 0.0;
        for &(a, b) in &self.bins {
            sum += cum_sum(bins, a, b);
        }

        // NORMALIZE USING RANGE [lower, upper]
        let mapped = self.lower + (self.upper - self.lower) * sum.clamp(0.0, 1.0);

        let value = (mapped * 255.0).clamp(0.0, 255.0) as u8;

        for &ch in &self.dmx_channels {
            dmx.insert(ch, value);
        }
    }
}

//
// ──────────────────────────────────────────────────────────────
// TOGGLE EFFECT (same on/off for all channels)
// ──────────────────────────────────────────────────────────────
//

pub struct ToggleEffect {
    pub dmx_channels: Vec<u16>,
    pub bins: Vec<(usize, usize)>,
    pub threshold: f32,
    pub on_value: u8,
    pub off_value: u8,
}

impl ToggleEffect {
    pub fn new(dmx_channels: Vec<u16>, bins: Vec<(usize, usize)>) -> Self {
        Self {
            dmx_channels,
            bins,
            threshold: 1.0,
            on_value: 255,
            off_value: 0,
        }
    }
}

impl Effect for ToggleEffect {
    fn apply(&self, bins: &[f32], dmx: &mut HashMap<u16, u8>) {

        let mut sum = 0.0;
        let mut count = 0.0;

        for &(a, b) in &self.bins {
            let slice = cum_sum(bins, a, b);
            sum += slice;
            count += (b - a + 1) as f32;
        }

        let avg = if count == 0.0 { 0.0 } else { sum / count };

        let val = if avg > self.threshold {
            self.on_value
        } else {
            self.off_value
        };

        for &ch in &self.dmx_channels {
            dmx.insert(ch, val);
        }
    }
}

//
// ──────────────────────────────────────────────────────────────
// SCENE TOGGLE EFFECT (per-channel on/off values)
// ──────────────────────────────────────────────────────────────
//

pub struct SceneToggleEffect {
    pub dmx_channels: HashMap<u16, (u8, u8)>, // channel → (on, off)
    pub bins: Vec<(usize, usize)>,
    pub threshold: f32,
}

impl SceneToggleEffect {
    pub fn new(dmx_channels: HashMap<u16, (u8, u8)>, bins: Vec<(usize, usize)>) -> Self {
        Self {
            dmx_channels,
            bins,
            threshold: 1.0,
        }
    }
}

impl Effect for SceneToggleEffect {
    fn apply(&self, bins: &[f32], dmx: &mut HashMap<u16, u8>) {

        let mut sum = 0.0;
        let mut count = 0.0;

        for &(a, b) in &self.bins {
            let slice = cum_sum(bins, a, b);
            sum += slice;
            count += (b - a + 1) as f32;
        }

        let avg = if count == 0.0 { 0.0 } else { sum / count };
        let active = avg > self.threshold;

        for (&ch, &(on, off)) in &self.dmx_channels {
            dmx.insert(ch, if active { on } else { off });
        }
    }
}

//
// ──────────────────────────────────────────────────────────────
// EFFECT SUITE
// ──────────────────────────────────────────────────────────────
//

#[derive(Serialize)]
pub struct DmxFrame {
    pub channels: HashMap<u16, u8>,
}

pub struct EffectSuite {
    pub effects: Vec<Box<dyn Effect + Send + Sync>>,
}

impl EffectSuite {
    pub fn new() -> Self {
        Self { effects: vec![] }
    }

    pub fn add<E: Effect + Send + Sync + 'static>(&mut self, eff: E) {
        self.effects.push(Box::new(eff));
    }

    /// Return a real JSON value, not a string  
    /// Ensure DMX values stay in range [0,255]
    pub fn process(&self, bins: &[f32]) -> serde_json::Value {
        let mut dmx: HashMap<u16, u8> = HashMap::new();

        // apply each effect; effects write f32 values for flexibility
        for eff in &self.effects {
            eff.apply(bins, &mut dmx);
        }

        // Wrap into a DMX frame
        serde_json::json!(DmxFrame {
            channels: dmx
        })
    }
}

