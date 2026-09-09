//! Vector scoring + combo bonuses + greedy 4–8 stack assembly.

use crate::catalog::{self, Dose, Family, Ingredient};
use crate::features::{self, cosine, FeatureVec};
use crate::persona::{existing_stack, vectorize, Goal, Persona, Training};
use crate::safety::{self, SafetyVerdict, Severity};

#[derive(Debug, Clone)]
pub struct ScoredItem {
    pub ingredient: &'static Ingredient,
    pub dose: Dose,
    pub cosine: f32,
    pub goal_bonus: f32,
    pub combo_bonus: f32,
    pub safety: SafetyVerdict,
    pub final_score: f32,
    pub why: String,
    pub safety_note: String,
    pub timing: &'static str,
}

#[derive(Debug, Clone)]
pub struct Recommendation {
    pub persona_id: String,
    pub persona_label: String,
    pub persona_summary: String,
    pub goals: String,
    pub vector: FeatureVec,
    pub vector_preview: Vec<(String, f32)>,
    pub stack: Vec<ScoredItem>,
    pub excluded: Vec<ScoredItem>,
    pub flags: Vec<String>,
    pub combo_notes: Vec<String>,
    pub catalog_notes: Vec<String>,
}

pub fn recommend(persona: &Persona) -> Recommendation {
    let vector = vectorize(persona);
    let mut scored: Vec<ScoredItem> = catalog::catalog()
        .iter()
        .map(|ing| score_one(persona, &vector, ing))
        .collect();

    apply_combo_bonuses(persona, &mut scored);

    let mut excluded: Vec<ScoredItem> = scored
        .iter()
        .filter(|s| s.safety.blocked() || s.safety.severity >= Severity::StrongPenalty)
        .cloned()
        .collect();
    excluded.sort_by(|a, b| {
        b.safety
            .severity
            .cmp(&a.safety.severity)
            .then(b.final_score.partial_cmp(&a.final_score).unwrap())
    });

    let mut candidates: Vec<ScoredItem> =
        scored.into_iter().filter(|s| !s.safety.blocked()).collect();
    candidates.sort_by(|a, b| b.final_score.partial_cmp(&a.final_score).unwrap());

    let stack = assemble_stack(persona, &candidates);
    let combo_notes = combo_notes_for(persona, &stack);
    let catalog_notes = catalog_notes(persona);

    Recommendation {
        persona_id: persona.id.to_string(),
        persona_label: persona.label.to_string(),
        persona_summary: persona.summary.to_string(),
        goals: persona.goal_list(),
        vector,
        vector_preview: features::nonzero_dims(&vector, 0.20),
        stack,
        excluded,
        flags: safety::persona_flags(persona),
        combo_notes,
        catalog_notes,
    }
}

fn score_one(persona: &Persona, vector: &FeatureVec, ing: &'static Ingredient) -> ScoredItem {
    let cos = cosine(vector, &ing.benefit);
    let goal_bonus = goal_alignment(persona, ing);
    let safety = safety::evaluate(persona, ing);
    let raw = (0.72 * cos + 0.28 * goal_bonus).clamp(0.0, 1.0);
    let final_score = (raw * safety.multiplier()).clamp(0.0, 1.0);
    let dose = pick_dose(persona, ing);
    let (why, timing) = explain(persona, ing, dose, cos);
    let safety_note = if safety.hits.is_empty() {
        "No specific contraindication fired for this persona.".into()
    } else {
        safety
            .hits
            .iter()
            .map(|h| format!("{} — {}", h.severity.label(), h.reason))
            .collect::<Vec<_>>()
            .join(" ")
    };

    ScoredItem {
        ingredient: ing,
        dose,
        cosine: cos,
        goal_bonus,
        combo_bonus: 0.0,
        safety,
        final_score,
        why,
        safety_note,
        timing,
    }
}

fn goal_alignment(p: &Persona, ing: &Ingredient) -> f32 {
    let mut s = 0.0f32;
    let mut n = 0.0f32;
    for g in &p.goals {
        n += 1.0;
        s += match (*g, ing.id) {
            (Goal::Sleep, "magnesium" | "theanine" | "ashwagandha") => 1.0,
            (Goal::Sleep, "taurine") => 0.45,
            (Goal::Sleep, "caffeine" | "green_tea") => 0.0,
            (Goal::Energy, "alcar") => 0.95,
            (Goal::Energy, "cordyceps") if p.trains() => 0.90,
            (Goal::Energy, "cordyceps") => 0.35,
            (Goal::Energy, "theanine") if p.caffeine.cups >= 2 => 0.70,
            (Goal::Energy, "creatine") if p.trains() => 0.70,
            (Goal::Energy, "taurine" | "alpha_gpc") => 0.55,
            (Goal::Energy, "himalayan_salt")
                if matches!(p.work, crate::persona::WorkPattern::LongHoursDriving)
                    || p.high_caffeine() =>
            {
                0.55
            }
            (Goal::Energy, "caffeine" | "green_tea")
                if p.high_caffeine() || p.has_goal(Goal::Sleep) =>
            {
                0.05
            }
            (Goal::Energy, "b12" | "b6") => 0.25,
            (Goal::Stress, "ashwagandha" | "theanine") => 1.0,
            (Goal::Stress, "magnesium") => 0.75,
            (Goal::Stress, "taurine") => 0.40,
            (Goal::HealthyAgeing, "creatine") => 1.0,
            (Goal::HealthyAgeing, "alcar" | "magnesium") => 0.80,
            (Goal::HealthyAgeing, "citrulline" | "alpha_gpc" | "vitamin_c") => 0.60,
            (Goal::HealthyAgeing, "taurine" | "zinc") => 0.45,
            (Goal::HealthyAgeing, "vitamin_d") => 0.70,
            (Goal::JointComfort, "vitamin_c" | "creatine" | "magnesium") => 0.80,
            (Goal::JointComfort, "citrulline" | "boron") => 0.55,
            _ => 0.15,
        };
    }
    if n == 0.0 {
        0.0
    } else {
        (s / n).clamp(0.0, 1.0)
    }
}

fn apply_combo_bonuses(p: &Persona, scored: &mut [ScoredItem]) {
    // Combos are "would these two work together in one sachet?" — CLD-9's
    // public copy emphasizes combinations, not isolated bottles.
    let high_caffeine = p.caffeine.cups >= 2;
    for item in scored.iter_mut() {
        let mut bonus = 0.0;
        if item.ingredient.id == "theanine" && high_caffeine {
            bonus += 0.08; // classic theanine + existing coffee
        }
        if item.ingredient.id == "magnesium" && p.has_goal(Goal::Sleep) {
            bonus += 0.04;
        }
        if item.ingredient.id == "taurine"
            && matches!(p.work, crate::persona::WorkPattern::LongHoursDriving)
        {
            bonus += 0.04;
        }
        if item.ingredient.id == "himalayan_salt"
            && (p.high_caffeine()
                || matches!(p.work, crate::persona::WorkPattern::LongHoursDriving))
        {
            bonus += 0.05;
        }
        if item.ingredient.id == "creatine" && p.trains() {
            bonus += 0.05;
        }
        if item.ingredient.id == "creatine" && p.has_goal(Goal::HealthyAgeing) {
            bonus += 0.06;
        }
        item.combo_bonus = bonus;
        if !item.safety.blocked() {
            item.final_score = (item.final_score + bonus).clamp(0.0, 1.0);
        }
    }
}

fn assemble_stack(p: &Persona, ranked: &[ScoredItem]) -> Vec<ScoredItem> {
    let mut selected: Vec<ScoredItem> = Vec::new();
    let target = target_size(p);

    for item in ranked {
        if selected.len() >= 8 {
            break;
        }
        if item.final_score < 0.10 && selected.len() >= 4 {
            continue;
        }
        if can_add(p, &selected, item) {
            selected.push(item.clone());
        }
        if selected.len() >= target {
            // Allow one extra high-confidence fit.
            let next_strong = ranked
                .iter()
                .filter(|c| !selected.iter().any(|s| s.ingredient.id == c.ingredient.id))
                .any(|c| c.final_score >= 0.42 && can_add(p, &selected, c));
            if !next_strong || selected.len() >= target + 1 {
                break;
            }
        }
    }

    if selected.len() < 4 {
        for item in ranked {
            if selected
                .iter()
                .any(|s| s.ingredient.id == item.ingredient.id)
            {
                continue;
            }
            selected.push(item.clone());
            if selected.len() >= 4 {
                break;
            }
        }
    }

    selected.sort_by(|a, b| b.final_score.partial_cmp(&a.final_score).unwrap());
    selected
}

fn target_size(p: &Persona) -> usize {
    // 6 is the sweet spot for a daily sachet (CLD-9: one packet vs 6–10 bottles).
    if p.osa_risk() {
        6
    } else if p.goals.len() >= 2 && p.trains() {
        7
    } else {
        6
    }
}

fn can_add(p: &Persona, selected: &[ScoredItem], item: &ScoredItem) -> bool {
    if selected
        .iter()
        .any(|s| s.ingredient.id == item.ingredient.id)
    {
        return false;
    }
    let stims = selected
        .iter()
        .filter(|s| s.ingredient.stimulant || s.ingredient.contains_caffeine)
        .count();
    if (item.ingredient.stimulant || item.ingredient.contains_caffeine) && stims >= 1 {
        return false;
    }
    let bulky = selected.iter().filter(|s| s.ingredient.bulky).count();
    if item.ingredient.bulky && bulky >= 2 {
        return false;
    }
    let vitamins = selected
        .iter()
        .filter(|s| s.ingredient.family == Family::Vitamin)
        .count();
    if item.ingredient.family == Family::Vitamin && vitamins >= 2 {
        return false;
    }
    // Don't fill a sleep/stress sachet with circulation botanicals.
    if (p.has_goal(Goal::Sleep) || p.has_goal(Goal::Stress))
        && item.ingredient.id == "ginkgo"
        && selected.len() >= 4
    {
        return false;
    }
    true
}

fn pick_dose(p: &Persona, ing: &Ingredient) -> Dose {
    let doses = ing.doses;
    if doses.len() == 1 {
        return doses[0];
    }
    match ing.id {
        "theanine" => {
            if p.high_caffeine()
                || p.caffeine.cups >= 2
                || p.sleep.wired_at_bedtime
                || p.has_goal(Goal::Stress)
            {
                doses[1]
            } else {
                doses[0]
            }
        }
        "magnesium" => {
            if p.has_goal(Goal::Sleep)
                || p.sleep.night_waking
                || p.sleep.wired_at_bedtime
                || p.has_goal(Goal::JointComfort)
            {
                doses[1]
            } else {
                doses[0]
            }
        }
        "ashwagandha" => {
            if p.has_goal(Goal::Stress) {
                doses[1]
            } else {
                doses[0]
            }
        }
        "alcar" => {
            if p.has_goal(Goal::Energy) && matches!(p.age_band, crate::persona::AgeBand::A55_64) {
                doses[1]
            } else {
                doses[0]
            }
        }
        "taurine" => {
            if p.high_caffeine() || matches!(p.work, crate::persona::WorkPattern::LongHoursDriving)
            {
                doses[1]
            } else {
                doses[0]
            }
        }
        "citrulline" => {
            if matches!(p.training, Training::MixedGym { .. }) {
                doses[1]
            } else {
                doses[0]
            }
        }
        "green_tea" => doses[0], // 250 mg — hepatotoxicity caution
        "caffeine" => doses[0],  // 100 mg if we ever recommend it
        "himalayan_salt" => {
            if p.high_caffeine() || matches!(p.work, crate::persona::WorkPattern::LongHoursDriving)
            {
                doses[1]
            } else {
                doses[0]
            }
        }
        "alpha_gpc" => {
            if p.has_goal(Goal::Energy) || p.has_goal(Goal::HealthyAgeing) {
                doses[1]
            } else {
                doses[0]
            }
        }
        _ => doses[0],
    }
}

fn explain(p: &Persona, ing: &Ingredient, dose: Dose, cos: f32) -> (String, &'static str) {
    let goals = p.goal_list();
    let why = match ing.id {
        "magnesium" => format!(
            "Goal fit ({goals}): magnesium is the catalog mineral for nerve/muscle recovery and the most defensible sleep-adjacent pick CLD-9 actually sells. Cosine {cos:.2} vs sleep-maintenance / recovery / joint dims. Form is malate (site Recovery label) — not glycinate — we do not invent a salt they do not offer. Dose {} (offered: 200 or 400 mg){}.",
            dose.display(),
            if dose.amount == 400 {
                "; 400 mg because sleep, night waking, wired evenings, or joint comfort is on the card"
            } else {
                "; 200 mg as the conservative offered step"
            }
        ),
        "theanine" => format!(
            "Goal fit ({goals}): L-theanine covers calm-focus and sleep-onset without adding stimulant load. Cosine {cos:.2}. Already drinking {} coffee(s), last dose {} — this is the standard pairing so existing caffeine is less jagged, not a reason to add more caffeine.",
            p.caffeine.cups,
            p.caffeine.last.as_str()
        ),
        "ashwagandha" => format!(
            "Goal fit ({goals}): KSM-66-class ashwagandha is the catalog's stress/recovery adaptogen ({}). Cosine {cos:.2} against stress + sleep-quality tags. Helpful for perceived stress; it does not treat perimenopause, HRT, or mood disorders.",
            dose.display()
        ),
        "alcar" => format!(
            "Goal fit ({goals}): Acetyl-L-carnitine {} is the non-stim mitochondrial / cognitive option. Cosine {cos:.2}. {}.",
            dose.display(),
            if p.has_goal(Goal::HealthyAgeing) {
                "Better-supported in older / fatigued adults than as a 'pre-workout' — fits healthy ageing cognition"
            } else {
                "Useful when the user wants energy but coffee is already doing the catecholamine job"
            }
        ),
        "cordyceps" => format!(
            "Goal fit ({goals}): Cordyceps 2000 mg is CLD-9's non-stim 'ATP / physical energy' pick. Cosine {cos:.2}. Evidence is limited/mixed — included as a stimulant alternative, not as a proven VO2 drug.",
        ),
        "creatine" => format!(
            "Goal fit ({goals}): Creatine monohydrate 5000 mg (only offered dose) is the strongest evidence-backed active in this catalog for power, training, and healthy ageing / sarcopenia. Cosine {cos:.2}. Not a sleep treatment.",
        ),
        "taurine" => format!(
            "Goal fit ({goals}): Taurine {} supports endurance/hydration and is a reasonable sachet partner for high coffee intake or long driving days. Cosine {cos:.2}. Modest evidence; very clean safety at 1–2 g.",
            dose.display()
        ),
        "citrulline" => format!(
            "Goal fit ({goals}): L-citrulline {} is the catalog's blood-flow / exercise amino. Cosine {cos:.2}. Picked at 3 g unless there is a real lifting stimulus (6 g). Joint comfort here is indirect (perfusion), not an anti-inflammatory claim.",
            dose.display()
        ),
        "alpha_gpc" => format!(
            "Goal fit ({goals}): Alpha-GPC {} is a choline donor for cognition / focus without caffeine. Cosine {cos:.2}. Evidence is limited; used when mental energy is needed and stimulants are off the table.",
            dose.display()
        ),
        "himalayan_salt" => format!(
            "Goal fit ({goals}): Himalayan pink salt {} is a small electrolyte bump (sodium), not a mineral miracle. Cosine {cos:.2} vs hydration. Makes sense with diuretic coffee load or long driving days.",
            dose.display()
        ),
        "vitamin_c" => format!(
            "Goal fit ({goals}): 100 mg ascorbic acid is a collagen-synthesis cofactor and the catalog's Immunity item. Cosine {cos:.2}. Honest evidence: useful as a cofactor, not a joint-pain cure, and not a cold-prevention megadose.",
        ),
        "zinc" => format!(
            "Goal fit ({goals}): 10 mg zinc glycinate is a modest wellness add. Cosine {cos:.2}. Only belongs if a multi isn't already covering it.",
        ),
        "b12" => format!(
            "Goal fit ({goals}): 200 mcg methylcobalamin is for deficiency risk, not a stimulant. Cosine {cos:.2}. Weak pick if recent bloods were normal or a multi is already in play.",
        ),
        "b6" => format!(
            "Goal fit ({goals}): 20 mg P5P is a high-multiple-of-RDA energy-metabolism cofactor. Cosine {cos:.2}. Easy to overstack with a multi.",
        ),
        "vitamin_d" => format!(
            "Goal fit ({goals}): 4000 IU D3 is a real bone/wellness dose (IOM UL). Cosine {cos:.2}. Never stacked on an existing D product in this engine.",
        ),
        "ginkgo" => format!(
            "Goal fit ({goals}): 120 mg ginkgo is the circulation/cognition botanical. Cosine {cos:.2}. Mixed dementia data; bleeding caution; not an OSA treatment.",
        ),
        "green_tea" => format!(
            "Goal fit ({goals}): Green tea extract {} (50% EGCG) is an antioxidant/energy botanical that still counts as caffeine. Cosine {cos:.2}.",
            dose.display()
        ),
        "caffeine" => format!(
            "Goal fit ({goals}): Catalog caffeine {} for alertness. Cosine {cos:.2}. Strong evidence — and usually the wrong add when sleep is a goal or coffee intake is already high.",
            dose.display()
        ),
        "boron" => format!(
            "Goal fit ({goals}): 10 mg boron citrate for bone / mineral metabolism. Cosine {cos:.2}. Limited evidence; hormone-metabolism caution with HRT.",
        ),
        "dlpa" => format!(
            "Goal fit ({goals}): 1000 mg DL-phenylalanine is the catalog mood item. Cosine {cos:.2}. Insufficient clinical evidence; MAOI contraindication.",
        ),
        _ => format!("Goal fit ({goals}). Cosine {cos:.2}."),
    };

    let timing = match ing.id {
        "magnesium" | "theanine" | "ashwagandha" => {
            if p.has_goal(Goal::Sleep) || p.sleep.wired_at_bedtime {
                "evening / with last meal (wind-down)"
            } else {
                "with a meal; evening if stress is the use-case"
            }
        }
        "alcar" | "alpha_gpc" | "cordyceps" | "caffeine" | "green_tea" | "b6" | "b12" => {
            "morning / early day (with food)"
        }
        "creatine" | "citrulline" | "taurine" | "himalayan_salt" => {
            "any consistent daily time; training days with the session if relevant"
        }
        _ => "with a meal (CLD-9 directions: 250–300 ml cold water)",
    };

    (why, timing)
}

fn combo_notes_for(p: &Persona, stack: &[ScoredItem]) -> Vec<String> {
    let ids: Vec<&str> = stack.iter().map(|s| s.ingredient.id).collect();
    let mut notes = Vec::new();
    if ids.contains(&"theanine") && p.caffeine.cups >= 1 {
        notes.push(format!(
            "Combo: L-theanine + the {} coffee(s) they already drink (not extra sachet caffeine). This is the best-supported pairing in the catalog.",
            p.caffeine.cups
        ));
    }
    if ids.contains(&"magnesium") && ids.contains(&"theanine") {
        notes.push(
            "Combo: magnesium + L-theanine as the sleep/stress core of the packet (mineral + calm-focus amino).".into(),
        );
    }
    if ids.contains(&"alcar") && ids.contains(&"alpha_gpc") {
        notes.push(
            "Combo: ALCAR + Alpha-GPC as a non-stim cognitive/energy pair (mitochondrial + choline).".into(),
        );
    }
    if ids.contains(&"creatine") && ids.contains(&"citrulline") {
        notes.push(
            "Combo: creatine + L-citrulline (power + blood flow) — only when ageing or training justifies the bulky powders.".into(),
        );
    }
    if ids.contains(&"taurine") && ids.contains(&"himalayan_salt") {
        notes.push(
            "Combo: taurine + pink salt as a light hydration/osmolyte pair for coffee + driving load.".into(),
        );
    }
    if ids.contains(&"creatine") && ids.contains(&"vitamin_c") {
        notes.push(
            "Combo: creatine (muscle/ageing) + vitamin C (collagen cofactor) for the healthy-ageing / joint brief — complementary, not synergistic magic.".into(),
        );
    }
    notes.push(format!(
        "Delivery: {} actives in one daily sachet ({}, {}). Other label ingredients are excipients, not recommendable actives.",
        stack.len(),
        catalog::SACHET,
        catalog::FLAVORS
    ));
    notes
}

fn catalog_notes(p: &Persona) -> Vec<String> {
    let mut n = Vec::new();
    let e = existing_stack(p);
    n.push(
        "Catalog lock: only the 20 actives and listed doses from cld9.ai/custom `initialIngredients`. No off-catalog bottles (no iron, no omega-3, no glycinate swap, no extra herbs)."
            .into(),
    );
    if e.omega3 {
        n.push(
            "Omega-3 stays as a separate product — it is not in the CLD-9 custom library.".into(),
        );
    }
    if p.hormones.periods_heavy {
        n.push(
            "Iron is not a CLD-9 custom-builder active. Heavy periods → clinician + labs, not a guessed iron sachet line.".into(),
        );
    }
    n
}

/// Side-by-side allergy demo: same persona, extra avoid-list tokens.
pub fn allergy_demo(base: &Persona, allergies: &[&str]) -> (Recommendation, Recommendation) {
    let before = recommend(base);
    let after = recommend(&base.with_allergies(allergies));
    (before, after)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persona::{all_personas, persona_a, persona_b, persona_c, persona_k};

    fn ids_of(r: &Recommendation) -> Vec<&str> {
        r.stack.iter().map(|s| s.ingredient.id).collect()
    }

    #[test]
    fn all_personas_get_4_to_8() {
        for p in all_personas() {
            let r = recommend(&p);
            assert!(
                (4..=8).contains(&r.stack.len()),
                "persona {} stack size {}",
                p.id,
                r.stack.len()
            );
        }
    }

    #[test]
    fn only_catalog_doses() {
        for p in all_personas() {
            let r = recommend(&p);
            for item in &r.stack {
                assert!(
                    item.ingredient.doses.contains(&item.dose),
                    "{} dose {:?} not offered",
                    item.ingredient.id,
                    item.dose
                );
            }
        }
    }

    #[test]
    fn sleep_personas_do_not_get_stims() {
        for p in [persona_a(), persona_b(), persona_c()] {
            let r = recommend(&p);
            assert!(
                r.stack
                    .iter()
                    .all(|s| !s.ingredient.contains_caffeine && !s.ingredient.stimulant),
                "persona {} received a stimulant",
                p.id
            );
        }
    }

    #[test]
    fn persona_k_does_not_get_vitamin_d() {
        let r = recommend(&persona_k());
        assert!(!ids_of(&r).contains(&"vitamin_d"));
    }

    #[test]
    fn persona_a_does_not_get_multi_micros() {
        let rec = recommend(&persona_a());
        let ids = ids_of(&rec);
        for banned in ["vitamin_d", "b6", "b12", "zinc"] {
            assert!(
                !ids.contains(&banned),
                "persona A stacked {banned} on a multi"
            );
        }
    }

    #[test]
    fn persona_b_flags_osa() {
        let r = recommend(&persona_b());
        assert!(r.flags.iter().any(|f| f.contains("OSA")));
    }

    #[test]
    fn ashwagandha_allergy_removes_it_and_keeps_size() {
        let p = persona_a();
        let (before, after) = allergy_demo(&p, &["ashwagandha"]);
        assert!(ids_of(&before).contains(&"ashwagandha"));
        assert!(!ids_of(&after).contains(&"ashwagandha"));
        assert!((4..=8).contains(&after.stack.len()));
        assert!(after
            .excluded
            .iter()
            .any(|e| e.ingredient.id == "ashwagandha" && e.safety.blocked()));
    }

    #[test]
    fn caffeine_allergy_excludes_green_tea_too() {
        let (_before, after) = allergy_demo(&persona_c(), &["caffeine"]);
        assert!(after
            .excluded
            .iter()
            .any(|e| e.ingredient.id == "green_tea" && e.safety.blocked()));
        assert!(after
            .excluded
            .iter()
            .any(|e| e.ingredient.id == "caffeine" && e.safety.blocked()));
    }
}
