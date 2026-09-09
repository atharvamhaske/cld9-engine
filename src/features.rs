//! Shared feature space for quiz/persona vectors and catalog-item vectors.
//!
//! Every dimension is a named benefit tag. Personas are encoded from quiz
//! answers; ingredients are encoded from their CLD-9 category + evidence-backed
//! use cases. Scoring is cosine similarity on this space, then safety rules.

pub const DIM: usize = 18;

pub const ENERGY_NONSTIM: usize = 0;
pub const ENERGY_STIM: usize = 1;
pub const SLEEP_ONSET: usize = 2;
pub const SLEEP_MAINT: usize = 3;
pub const SLEEP_QUALITY: usize = 4;
pub const STRESS: usize = 5;
pub const CALM_FOCUS: usize = 6;
pub const COGNITION: usize = 7;
pub const PERFORMANCE: usize = 8;
pub const POWER: usize = 9;
pub const ENDURANCE: usize = 10;
pub const HYDRATION: usize = 11;
pub const AGEING: usize = 12;
pub const JOINT: usize = 13;
pub const RECOVERY: usize = 14;
pub const WELLNESS: usize = 15;
pub const MOOD: usize = 16;
pub const CIRCULATION: usize = 17;

pub const FEATURE_NAMES: [&str; DIM] = [
    "energy_nonstim",
    "energy_stim",
    "sleep_onset",
    "sleep_maintenance",
    "sleep_quality",
    "stress",
    "calm_focus",
    "cognition",
    "performance",
    "power",
    "endurance",
    "hydration",
    "healthy_ageing",
    "joint_comfort",
    "recovery",
    "wellness",
    "mood",
    "circulation",
];

pub type FeatureVec = [f32; DIM];

pub fn zero() -> FeatureVec {
    [0.0; DIM]
}

/// Cosine similarity in [0, 1] after clamping a negative dot product to 0.
/// Identical vectors → 1. Zero vectors → 0.
pub fn cosine(a: &FeatureVec, b: &FeatureVec) -> f32 {
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for i in 0..DIM {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    let denom = na.sqrt() * nb.sqrt();
    if denom == 0.0 {
        0.0
    } else {
        (dot / denom).clamp(0.0, 1.0)
    }
}

pub fn clamp_unit(v: &mut FeatureVec) {
    for x in v.iter_mut() {
        *x = x.clamp(0.0, 1.0);
    }
}

/// Human-readable dump of non-trivial dimensions (for CLI walkthroughs).
pub fn nonzero_dims(v: &FeatureVec, threshold: f32) -> Vec<(String, f32)> {
    FEATURE_NAMES
        .iter()
        .enumerate()
        .filter_map(|(i, name)| {
            if v[i] >= threshold {
                Some((name.to_string(), v[i]))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_vectors_cosine_one() {
        let mut v = zero();
        v[ENERGY_NONSTIM] = 0.8;
        v[SLEEP_QUALITY] = 0.6;
        assert!((cosine(&v, &v) - 1.0).abs() < 1e-5);
    }

    #[test]
    fn orthogonal_vectors_cosine_zero() {
        let mut a = zero();
        let mut b = zero();
        a[ENERGY_STIM] = 1.0;
        b[SLEEP_MAINT] = 1.0;
        assert!(cosine(&a, &b) < 1e-5);
    }
}
