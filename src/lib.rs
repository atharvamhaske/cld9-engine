//! CLD-9-style personalized daily-sachet recommendation engine.
//!
//! Pipeline: structured quiz/persona → feature vector → cosine + goal bonus
//! against the public custom-builder catalog → safety hard-filters / penalties
//! → ranked 4–8 stack with dose, evidence, and safety notes.

pub mod catalog;
pub mod engine;
pub mod features;
pub mod persona;
pub mod report;
pub mod safety;
pub mod tui;

pub use engine::{allergy_demo, recommend, Recommendation};
pub use persona::{all_personas, persona_by_id, Persona};
