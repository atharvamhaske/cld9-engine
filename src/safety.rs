//! Safety layer: allergies, medications, conditions, and existing-supplement
//! dedup. This runs *before* ranking (hard blocks) and *during* scoring
//! (penalties). Goal matching never overrides a hard block.

use crate::catalog::{Family, Ingredient};
use crate::persona::{existing_stack, ExistingStack, Goal, Persona};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    NoteOnly,
    SoftPenalty,
    ModeratePenalty,
    StrongPenalty,
    HardBlock,
}

impl Severity {
    pub fn multiplier(self) -> f32 {
        match self {
            Severity::HardBlock => 0.0,
            Severity::StrongPenalty => 0.12,
            Severity::ModeratePenalty => 0.40,
            Severity::SoftPenalty => 0.70,
            Severity::NoteOnly => 1.0,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Severity::HardBlock => "HARD BLOCK",
            Severity::StrongPenalty => "STRONG PENALTY",
            Severity::ModeratePenalty => "MODERATE PENALTY",
            Severity::SoftPenalty => "SOFT PENALTY",
            Severity::NoteOnly => "NOTE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SafetyHit {
    pub severity: Severity,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct SafetyVerdict {
    pub severity: Severity,
    pub hits: Vec<SafetyHit>,
}

impl SafetyVerdict {
    pub fn blocked(&self) -> bool {
        self.severity == Severity::HardBlock
    }

    pub fn multiplier(&self) -> f32 {
        self.severity.multiplier()
    }

    pub fn reasons(&self) -> Vec<String> {
        self.hits.iter().map(|h| h.reason.clone()).collect()
    }
}

fn hit(severity: Severity, reason: impl Into<String>) -> SafetyHit {
    SafetyHit {
        severity,
        reason: reason.into(),
    }
}

fn norm(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn allergy_match(token: &str, item: &Ingredient) -> bool {
    let t = norm(token);
    if t.is_empty() {
        return false;
    }
    let mut keys: Vec<String> = item.allergen_keys.iter().map(|k| norm(k)).collect();
    keys.push(norm(item.id));
    keys.push(norm(item.name));
    if let Some(alias) = item.alias {
        keys.push(norm(alias));
    }
    keys.iter()
        .any(|k| k == &t || k.contains(&t) || t.contains(k))
}

/// Caffeine allergy / sensitivity also excludes green-tea extract (residual caffeine).
fn caffeine_family(item: &Ingredient) -> bool {
    item.contains_caffeine || item.id == "caffeine" || item.id == "green_tea"
}

pub fn evaluate(persona: &Persona, item: &Ingredient) -> SafetyVerdict {
    let mut hits: Vec<SafetyHit> = Vec::new();
    let existing = existing_stack(persona);

    for allergy in &persona.allergies {
        if allergy_match(allergy, item)
            || (norm(allergy).contains("caffeine") && caffeine_family(item))
        {
            hits.push(hit(
                Severity::HardBlock,
                format!(
                    "Allergy / avoid list match on `{allergy}` → {} is excluded (hard filter).",
                    item.name
                ),
            ));
        }
    }

    if persona.pregnancy() && item.id == "ashwagandha" {
        hits.push(hit(
            Severity::HardBlock,
            "Pregnancy flag: ashwagandha is avoided (standard safety caution; not a prenatal).",
        ));
    }

    if persona.blood_thinners() && (item.id == "ginkgo" || item.id == "green_tea") {
        hits.push(hit(
            Severity::HardBlock,
            format!(
                "{} has antiplatelet / bleeding-risk signals and is blocked with anticoagulant or blood-thinner medications.",
                item.name
            ),
        ));
    }

    if persona.maoi() && item.id == "dlpa" {
        hits.push(hit(
            Severity::HardBlock,
            "DL-Phenylalanine is blocked with MAOI medication (catecholamine precursor interaction).",
        ));
    }

    stimulant_and_sleep_rules(persona, item, &mut hits);
    hormone_and_hrt_rules(persona, item, &mut hits);
    existing_supplement_rules(persona, item, &existing, &mut hits);
    condition_notes(persona, item, &mut hits);

    let severity = hits
        .iter()
        .map(|h| h.severity)
        .max()
        .unwrap_or(Severity::NoteOnly);

    SafetyVerdict { severity, hits }
}

fn stimulant_and_sleep_rules(p: &Persona, item: &Ingredient, hits: &mut Vec<SafetyHit>) {
    let stim = item.stimulant || item.contains_caffeine;
    if !stim {
        return;
    }

    if p.osa_risk() {
        hits.push(hit(
            Severity::HardBlock,
            format!(
                "{} is a stimulant / caffeine source. Persona has snoring + witnessed breathing pauses + unrefreshing sleep — possible OSA. Stimulants are not a treatment and can worsen fragmented sleep. See a clinician; supplements are adjunct at best.",
                item.name
            ),
        ));
        return;
    }

    if p.has_goal(Goal::Sleep)
        && (p.high_caffeine() || p.caffeine.last.is_late() || p.sleep.wired_at_bedtime)
    {
        hits.push(hit(
            Severity::HardBlock,
            format!(
                "{} blocked: sleep is a goal and caffeine load is already high ({} coffee(s), last dose {}). Prefer magnesium / L-theanine / sleep-hygiene timing over more stimulant.",
                item.name,
                p.caffeine.cups,
                p.caffeine.last.as_str()
            ),
        ));
        return;
    }

    if p.has_goal(Goal::Sleep) && p.caffeine.cups >= 2 {
        hits.push(hit(
            Severity::HardBlock,
            format!(
                "{} blocked: sleep is a primary goal and the user already drinks {} coffees/day. Adding catalog caffeine (100–300 mg) or caffeinated green tea extract fights the sleep goal.",
                item.name, p.caffeine.cups
            ),
        ));
        return;
    }

    if p.sleep.wired_at_bedtime || (p.high_caffeine() && p.caffeine.last.is_late()) {
        hits.push(hit(
            Severity::HardBlock,
            format!(
                "{} blocked: wired-at-bedtime and/or late, high caffeine intake ({} cups, last {}). More stimulant is the opposite intervention.",
                item.name,
                p.caffeine.cups,
                p.caffeine.last.as_str()
            ),
        ));
        return;
    }

    if p.high_caffeine() {
        hits.push(hit(
            Severity::StrongPenalty,
            format!(
                "Already ~{} coffees/day. Extra {} is heavily down-ranked; pair existing caffeine with L-theanine instead.",
                p.caffeine.cups, item.name
            ),
        ));
    } else if p.caffeine.cups >= 2 && item.id == "caffeine" {
        hits.push(hit(
            Severity::StrongPenalty,
            "Two daily coffees already; catalog caffeine powder is usually redundant.",
        ));
    } else if p.caffeine.cups >= 1 && item.id == "green_tea" {
        hits.push(hit(
            Severity::SoftPenalty,
            "Green tea extract still contributes caffeine + EGCG. Keep only if the ageing/antioxidant case is strong; prefer the 250 mg option.",
        ));
    }
}

fn hormone_and_hrt_rules(p: &Persona, item: &Ingredient, hits: &mut Vec<SafetyHit>) {
    if p.on_hrt() {
        match item.id {
            "ashwagandha" => hits.push(hit(
                Severity::StrongPenalty,
                "On HRT: ashwagandha can nudge thyroid / HPA-axis markers. Do not use it to 'manage' menopause or to second-guess prescribed HRT — clinician-managed hormones win.",
            )),
            "boron" => hits.push(hit(
                Severity::StrongPenalty,
                "On HRT: boron influences estrogen/testosterone metabolism. 10 mg is a high supplemental dose. Not stacked on top of clinician-managed HRT without their OK.",
            )),
            "ginkgo" => hits.push(hit(
                Severity::ModeratePenalty,
                "On HRT: ginkgo has bleeding-risk signals. Extra caution if the HRT regimen or other meds affect clotting; we down-rank rather than auto-include.",
            )),
            _ => {}
        }
    }

    if p.hormones.periods_heavy && item.id == "ashwagandha" {
        hits.push(hit(
            Severity::NoteOnly,
            "Heavy / irregular periods are a clinician question (fibroids, thyroid, iron status). Ashwagandha is not a treatment for menorrhagia.",
        ));
    }
}

fn existing_supplement_rules(
    p: &Persona,
    item: &Ingredient,
    existing: &ExistingStack,
    hits: &mut Vec<SafetyHit>,
) {
    let dup = match item.id {
        "vitamin_d" if existing.vitamin_d => Some((
            Severity::HardBlock,
            if existing.multivitamin {
                "Already taking a multivitamin (typically includes vitamin D). Catalog dose is 4000 IU — the IOM UL. Do not stack another 4000 IU on top without a 25-OH-D level and a plan."
            } else {
                "Already taking vitamin D. Catalog only offers 4000 IU — stacking that on an existing D supplement is a safety miss, not a wellness upgrade."
            },
        )),
        "vitamin_c" if existing.vitamin_c => Some((
            Severity::StrongPenalty,
            "Vitamin C is already covered (multi or dedicated C). 100 mg more is low-risk but redundant — down-ranked unless a collagen/joint case is unusually strong and the multi is untrusted.",
        )),
        "b6" if existing.b6 => Some((
            Severity::StrongPenalty,
            "B6 already covered by the multivitamin. Catalog dose is 20 mg P5P (many times the RDA). No indication to restack.",
        )),
        "b12" if existing.b12 => Some((
            Severity::StrongPenalty,
            "B12 already covered by the multivitamin. Extra methylcobalamin is not an energy drug when intake is adequate.",
        )),
        "zinc" if existing.zinc => Some((
            Severity::StrongPenalty,
            "Zinc already covered by the multivitamin. Stacking 10 mg glycinate on a typical multi can be unnecessary and competes with copper over time.",
        )),
        "magnesium" if existing.magnesium => Some((
            Severity::HardBlock,
            "Already taking magnesium — do not duplicate the same mineral in the sachet without adjusting the existing product.",
        )),
        "creatine" if existing.creatine => Some((
            Severity::HardBlock,
            "Already taking creatine. Catalog only offers 5000 mg; adding another 5 g is a duplicate, not a new active.",
        )),
        "ashwagandha" if existing.ashwagandha => Some((
            Severity::HardBlock,
            "Already taking ashwagandha — do not double the adaptogen.",
        )),
        "theanine" if existing.theanine => Some((
            Severity::HardBlock,
            "Already taking L-theanine — do not duplicate.",
        )),
        _ => None,
    };
    if let Some((sev, reason)) = dup {
        hits.push(hit(sev, reason));
    }

    // Multi: remaining wellness micros that weren't exact-flagged still get a nudge.
    if existing.multivitamin && item.family == Family::Vitamin && item.id != "vitamin_d" {
        if !hits.iter().any(|h| h.severity >= Severity::StrongPenalty) {
            hits.push(hit(
                Severity::ModeratePenalty,
                "Takes a multivitamin — extra stand-alone vitamins are down-ranked unless a specific gap is argued.",
            ));
        }
    }

    let _ = p;
}

fn condition_notes(p: &Persona, item: &Ingredient, hits: &mut Vec<SafetyHit>) {
    if p.osa_risk() && !item.stimulant {
        if item.id == "magnesium" || item.id == "theanine" || item.id == "taurine" {
            hits.push(hit(
                Severity::NoteOnly,
                "Possible OSA: this item is an adjunct for comfort / wind-down at best. It does not treat apneas. Medical evaluation (sleep study) comes first.",
            ));
        }
    }

    if p.hormones.periods_heavy {
        // Iron is NOT in the CLD-9 catalog — say so rather than inventing it.
        if item.id == "b12" || item.id == "vitamin_c" {
            hits.push(hit(
                Severity::NoteOnly,
                "Heavy periods: do not assume iron deficiency and do not invent iron (not in the CLD-9 catalog). Ferritin/CBC belong with a clinician; the multi may already contain a small iron dose — we will not stack a guess.",
            ));
        }
    }
}

/// Persona-level banners that are independent of any single ingredient.
pub fn persona_flags(p: &Persona) -> Vec<String> {
    let mut flags = Vec::new();
    let existing = existing_stack(p);

    flags.push(
        "Not medical advice. Educational take-home only — not a diagnosis, prescription, or care plan."
            .into(),
    );

    if p.osa_risk() {
        flags.push(
            "PERSONA B / OSA FLAG: Snoring + witnessed breathing pauses + unrefreshing sleep after 8h+ in bed is a classic screen for obstructive sleep apnea (higher-weight, middle-aged, sedentary). This is a clinician / sleep-study question. Supplements do not treat OSA. Anything below is an adjunct for daytime comfort at best and must not delay care."
                .into(),
        );
    }

    if p.hormones.periods_heavy || p.hormones.cycles_irregular {
        flags.push(
            "PERSONA A / CYCLE FLAG: Cycles becoming irregular and periods heavy / heavier is not something a sachet should 'treat'. See a clinician (PMB/AUB workup, thyroid, ferritin). Iron is not in the CLD-9 catalog — we will not invent it, and we will not assume deficiency on top of a multivitamin."
                .into(),
        );
    }

    if p.on_hrt() {
        flags.push(
            "PERSONA K / HRT FLAG: Hormone replacement is clinician-managed. This engine will not contradict prescribed HRT, will not use ashwagandha or boron as hormone 'support', and will not restack 4000 IU vitamin D on top of an existing D supplement."
                .into(),
        );
    }

    if existing.omega3 {
        flags.push(
            "CATALOG GAP: Already taking omega-3. EPA/DHA is not one of the 20 CLD-9 custom-builder actives, so we cannot fold it into the sachet — keep the existing product. This is a feature of their public catalog, not a knock on omega-3."
                .into(),
        );
    }

    if existing.multivitamin {
        flags.push(
            "Already takes a multivitamin: B6, B12, C, D, and zinc are treated as covered and are hard-blocked or heavily down-ranked so the sachet does not silently double a multi."
                .into(),
        );
    }

    if existing.vitamin_d && !existing.multivitamin {
        flags.push(
            "Already takes vitamin D: catalog only offers 4000 IU cholecalciferol (IOM UL). We will not add another 4000 IU on top."
                .into(),
        );
    }

    if p.high_caffeine() || (p.has_goal(Goal::Sleep) && p.caffeine.cups >= 2) {
        flags.push(format!(
            "Caffeine context: {} coffee(s)/day, last dose {}. Stimulants (caffeine powder, green tea extract) are blocked or heavily penalized; L-theanine / magnesium are preferred.",
            p.caffeine.cups,
            p.caffeine.last.as_str()
        ));
    }

    if p.sleep.wired_at_bedtime {
        flags.push(
            "Wired at bedtime + late caffeine: first intervention is timing (no coffee after early afternoon), not another capsule. The sachet can only support that."
                .into(),
        );
    }

    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::by_id;
    use crate::persona::{persona_a, persona_b, persona_c, persona_k};

    #[test]
    fn caffeine_allergy_blocks_coffee_and_green_tea() {
        let p = persona_c().with_allergies(&["caffeine"]);
        assert!(evaluate(&p, by_id("caffeine").unwrap()).blocked());
        assert!(evaluate(&p, by_id("green_tea").unwrap()).blocked());
        assert!(!evaluate(&p, by_id("theanine").unwrap()).blocked());
    }

    #[test]
    fn ashwagandha_allergy_blocks_only_ashwagandha() {
        let p = persona_a().with_allergies(&["ashwagandha"]);
        assert!(evaluate(&p, by_id("ashwagandha").unwrap()).blocked());
        assert!(!evaluate(&p, by_id("magnesium").unwrap()).blocked());
    }

    #[test]
    fn existing_d_hard_blocks_4000iu() {
        let k = persona_k();
        assert!(evaluate(&k, by_id("vitamin_d").unwrap()).blocked());
        let a = persona_a();
        assert!(evaluate(&a, by_id("vitamin_d").unwrap()).blocked());
    }

    #[test]
    fn persona_b_blocks_stims() {
        let b = persona_b();
        assert!(evaluate(&b, by_id("caffeine").unwrap()).blocked());
        assert!(evaluate(&b, by_id("green_tea").unwrap()).blocked());
        assert!(!evaluate(&b, by_id("magnesium").unwrap()).blocked());
    }

    #[test]
    fn hrt_penalizes_ashwagandha_and_boron() {
        let k = persona_k();
        let a = evaluate(&k, by_id("ashwagandha").unwrap());
        let b = evaluate(&k, by_id("boron").unwrap());
        assert_eq!(a.severity, Severity::StrongPenalty);
        assert_eq!(b.severity, Severity::StrongPenalty);
        assert!(!a.blocked());
    }
}
