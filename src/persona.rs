//! Structured quiz/persona records and explicit vectorization into the
//! shared feature space.

use crate::features::{
    self, clamp_unit, zero, FeatureVec, AGEING, CALM_FOCUS, CIRCULATION, COGNITION, ENDURANCE,
    ENERGY_NONSTIM, ENERGY_STIM, HYDRATION, JOINT, MOOD, PERFORMANCE, POWER, RECOVERY, SLEEP_MAINT,
    SLEEP_ONSET, SLEEP_QUALITY, STRESS, WELLNESS,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgeBand {
    A25_34,
    A45_54,
    A55_64,
}

impl AgeBand {
    pub fn as_str(self) -> &'static str {
        match self {
            AgeBand::A25_34 => "25-34",
            AgeBand::A45_54 => "45-54",
            AgeBand::A55_64 => "55-64",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sex {
    Female,
    Male,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diet {
    Omnivore,
    Pescatarian,
}

impl Diet {
    pub fn as_str(self) -> &'static str {
        match self {
            Diet::Omnivore => "omnivore",
            Diet::Pescatarian => "pescatarian",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkPattern {
    StandardHours,
    LongHoursDriving,
    LongHoursSeated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Training {
    None,
    MixedGym { sessions_per_week: u8 },
    YogaWalking { sessions_per_week: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaffeineLast {
    Morning,
    EarlyAfternoon,
    MidAfternoon,
    LateAfternoon,
}

impl CaffeineLast {
    pub fn as_str(self) -> &'static str {
        match self {
            CaffeineLast::Morning => "morning",
            CaffeineLast::EarlyAfternoon => "early afternoon",
            CaffeineLast::MidAfternoon => "mid-afternoon",
            CaffeineLast::LateAfternoon => "late afternoon",
        }
    }

    pub fn is_late(self) -> bool {
        matches!(
            self,
            CaffeineLast::MidAfternoon | CaffeineLast::LateAfternoon
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyWeight {
    Typical,
    Higher,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Goal {
    Energy,
    Sleep,
    Stress,
    HealthyAgeing,
    JointComfort,
}

impl Goal {
    pub fn as_str(self) -> &'static str {
        match self {
            Goal::Energy => "energy",
            Goal::Sleep => "sleep",
            Goal::Stress => "stress",
            Goal::HealthyAgeing => "healthy ageing",
            Goal::JointComfort => "joint comfort",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SleepProfile {
    pub duration: &'static str,
    pub night_waking: bool,
    pub night_waking_detail: Option<&'static str>,
    pub unrefreshed: bool,
    pub snores: bool,
    pub witnessed_apneas: bool,
    pub wired_at_bedtime: bool,
    pub vasomotor_symptoms: bool,
    pub occasional_early_waking: bool,
}

#[derive(Debug, Clone)]
pub struct HormoneStatus {
    pub cycles_irregular: bool,
    pub periods_heavy: bool,
    pub perimenopause: bool,
    pub post_menopause: bool,
    pub on_hrt: bool,
}

#[derive(Debug, Clone)]
pub struct CaffeineIntake {
    pub cups: u8,
    pub last: CaffeineLast,
}

#[derive(Debug, Clone)]
pub struct Persona {
    pub id: &'static str,
    pub label: &'static str,
    pub summary: &'static str,
    pub age_band: AgeBand,
    pub sex: Sex,
    pub diet: Diet,
    pub occupation: &'static str,
    pub work: WorkPattern,
    pub sleep: SleepProfile,
    pub training: Training,
    pub caffeine: CaffeineIntake,
    pub hormones: HormoneStatus,
    pub medications: Vec<String>,
    pub existing_supplements: Vec<String>,
    pub allergies: Vec<String>,
    pub conditions: Vec<String>,
    pub goals: Vec<Goal>,
    pub body_weight: BodyWeight,
}

impl Persona {
    pub fn with_allergies(&self, extra: &[&str]) -> Persona {
        let mut p = self.clone();
        for a in extra {
            let key = a.to_string();
            if !p.allergies.iter().any(|x| x.eq_ignore_ascii_case(&key)) {
                p.allergies.push(key);
            }
        }
        p
    }

    pub fn goal_list(&self) -> String {
        self.goals
            .iter()
            .map(|g| g.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn has_goal(&self, g: Goal) -> bool {
        self.goals.contains(&g)
    }

    pub fn trains(&self) -> bool {
        !matches!(self.training, Training::None)
    }

    pub fn osa_risk(&self) -> bool {
        self.sleep.snores && self.sleep.witnessed_apneas
            || self.conditions.iter().any(|c| {
                let c = c.to_lowercase();
                c.contains("osa") || c.contains("apnea") || c.contains("apnoea")
            })
    }

    pub fn high_caffeine(&self) -> bool {
        self.caffeine.cups >= 3
    }

    pub fn on_hrt(&self) -> bool {
        self.hormones.on_hrt
            || self.medications.iter().any(|m| {
                let m = m.to_lowercase();
                m.contains("hrt")
                    || m.contains("estrogen")
                    || m.contains("oestrogen")
                    || m.contains("estradiol")
                    || m.contains("progesterone")
            })
    }

    pub fn pregnancy(&self) -> bool {
        self.conditions.iter().any(|c| {
            let c = c.to_lowercase();
            c.contains("pregnan")
        })
    }

    pub fn blood_thinners(&self) -> bool {
        self.medications.iter().any(|m| {
            let m = m.to_lowercase();
            m.contains("warfarin")
                || m.contains("apixaban")
                || m.contains("rivaroxaban")
                || m.contains("dabigatran")
                || m.contains("clopidogrel")
                || m.contains("anticoag")
                || m.contains("blood thinner")
                || m.contains("aspirin")
        })
    }

    pub fn maoi(&self) -> bool {
        self.medications.iter().any(|m| {
            let m = m.to_lowercase();
            m.contains("maoi")
                || m.contains("phenelzine")
                || m.contains("tranylcypromine")
                || m.contains("selegiline")
                || m.contains("isocarboxazid")
        })
    }
}

/// Parsed view of what the user already takes. Used by the safety layer
/// to deduplicate / downrank instead of blindly restacking.
#[derive(Debug, Clone, Default)]
pub struct ExistingStack {
    pub multivitamin: bool,
    pub vitamin_d: bool,
    pub vitamin_c: bool,
    pub b6: bool,
    pub b12: bool,
    pub zinc: bool,
    pub magnesium: bool,
    pub omega3: bool,
    pub creatine: bool,
    pub ashwagandha: bool,
    pub theanine: bool,
    pub raw: Vec<String>,
}

pub fn existing_stack(p: &Persona) -> ExistingStack {
    let mut e = ExistingStack {
        raw: p.existing_supplements.clone(),
        ..Default::default()
    };
    for raw in &p.existing_supplements {
        let s = raw.to_lowercase();
        if s.contains("multi") || s.contains("centrum") || s.contains("one-a-day") {
            e.multivitamin = true;
            e.vitamin_d = true;
            e.vitamin_c = true;
            e.b6 = true;
            e.b12 = true;
            e.zinc = true;
        }
        if s.contains("vitamin d")
            || s.contains("vit d")
            || s.contains("d3")
            || s.contains("cholecalciferol")
        {
            e.vitamin_d = true;
        }
        if s.contains("vitamin c") || s.contains("ascorb") {
            e.vitamin_c = true;
        }
        if s.contains("b12") || s.contains("b-12") || s.contains("cobalamin") {
            e.b12 = true;
        }
        if s.contains("b6") || s.contains("b-6") || s.contains("p5p") || s.contains("pyridox") {
            e.b6 = true;
        }
        if s.contains("zinc") {
            e.zinc = true;
        }
        if s.contains("magnesium") {
            e.magnesium = true;
        }
        if s.contains("omega") || s.contains("fish oil") || s.contains("dha") || s.contains("epa") {
            e.omega3 = true;
        }
        if s.contains("creatine") {
            e.creatine = true;
        }
        if s.contains("ashwagandha") || s.contains("ksm") {
            e.ashwagandha = true;
        }
        if s.contains("theanine") {
            e.theanine = true;
        }
    }
    e
}

/// Encode quiz/persona answers as an 18-dimensional benefit-need vector.
///
/// Order of influence:
/// 1. Primary goals (largest weights)
/// 2. Sleep / caffeine / training overlays
/// 3. Life stage (perimenopause, post-menopause, age band)
/// 4. Work pattern / body weight
///
/// Stimulant-need is driven toward 0 when sleep is a goal or caffeine is
/// already high — the safety layer then hard-blocks leftover stimulant hits.
pub fn vectorize(p: &Persona) -> FeatureVec {
    let mut v = zero();

    for g in &p.goals {
        match g {
            Goal::Energy => {
                v[ENERGY_NONSTIM] += 0.85;
                v[ENERGY_STIM] += 0.20;
                v[ENDURANCE] += 0.25;
                v[COGNITION] += 0.20;
            }
            Goal::Sleep => {
                v[SLEEP_QUALITY] += 0.80;
                v[SLEEP_ONSET] += 0.50;
                v[SLEEP_MAINT] += 0.50;
                v[CALM_FOCUS] += 0.35;
                v[ENERGY_STIM] -= 0.55;
            }
            Goal::Stress => {
                v[STRESS] += 0.95;
                v[CALM_FOCUS] += 0.55;
                v[SLEEP_ONSET] += 0.25;
                v[RECOVERY] += 0.20;
            }
            Goal::HealthyAgeing => {
                v[AGEING] += 0.95;
                v[JOINT] += 0.30;
                v[COGNITION] += 0.40;
                v[POWER] += 0.40;
                v[WELLNESS] += 0.25;
                v[CIRCULATION] += 0.20;
            }
            Goal::JointComfort => {
                v[JOINT] += 0.95;
                v[RECOVERY] += 0.30;
                v[AGEING] += 0.25;
                v[CIRCULATION] += 0.15;
            }
        }
    }

    if p.sleep.night_waking {
        v[SLEEP_MAINT] += 0.40;
        v[SLEEP_QUALITY] += 0.15;
    }
    if p.sleep.occasional_early_waking {
        v[SLEEP_MAINT] += 0.15;
    }
    if p.sleep.unrefreshed {
        v[SLEEP_QUALITY] += 0.25;
        // Unrefreshing sleep is a symptom, not a request for sedatives.
        v[ENERGY_NONSTIM] += 0.15;
    }
    if p.sleep.wired_at_bedtime {
        v[SLEEP_ONSET] += 0.40;
        v[CALM_FOCUS] += 0.25;
        v[STRESS] += 0.15;
        v[ENERGY_STIM] = 0.0;
    }
    if p.sleep.vasomotor_symptoms {
        v[SLEEP_MAINT] += 0.15;
        v[RECOVERY] += 0.10;
    }

    match p.caffeine.cups {
        0 => {}
        1 => v[ENERGY_STIM] -= 0.10,
        2 => {
            v[ENERGY_STIM] -= 0.25;
            v[CALM_FOCUS] += 0.10;
        }
        n if n >= 3 => {
            v[ENERGY_STIM] = 0.0;
            v[CALM_FOCUS] += 0.25;
            v[HYDRATION] += 0.15;
        }
        _ => {}
    }
    if p.caffeine.last.is_late() {
        v[ENERGY_STIM] = 0.0;
        v[SLEEP_ONSET] += 0.15;
    }

    match p.training {
        Training::None => {
            if matches!(p.age_band, AgeBand::A55_64) {
                v[POWER] += 0.15;
            }
        }
        Training::MixedGym { sessions_per_week } if sessions_per_week >= 3 => {
            v[PERFORMANCE] += 0.35;
            v[POWER] += 0.35;
            v[RECOVERY] += 0.30;
        }
        Training::MixedGym { .. } => {
            v[PERFORMANCE] += 0.20;
            v[POWER] += 0.20;
            v[RECOVERY] += 0.15;
        }
        Training::YogaWalking { sessions_per_week } if sessions_per_week >= 3 => {
            v[RECOVERY] += 0.20;
            v[JOINT] += 0.15;
            v[PERFORMANCE] += 0.15;
            v[CIRCULATION] += 0.15;
        }
        Training::YogaWalking { .. } => {
            v[RECOVERY] += 0.10;
            v[JOINT] += 0.10;
        }
    }

    match p.work {
        WorkPattern::LongHoursDriving => {
            v[HYDRATION] += 0.30;
            v[COGNITION] += 0.15;
            v[ENDURANCE] += 0.10;
        }
        WorkPattern::LongHoursSeated => {
            v[CIRCULATION] += 0.15;
            v[STRESS] += 0.10;
        }
        WorkPattern::StandardHours => {}
    }

    if p.hormones.perimenopause || p.hormones.cycles_irregular {
        v[SLEEP_MAINT] += 0.10;
        v[STRESS] += 0.10;
        v[RECOVERY] += 0.10;
    }
    if p.hormones.post_menopause {
        v[AGEING] += 0.10;
        v[JOINT] += 0.10;
        v[POWER] += 0.10;
    }

    if matches!(p.age_band, AgeBand::A55_64) {
        v[AGEING] += 0.10;
        v[WELLNESS] += 0.05;
    }
    if matches!(p.body_weight, BodyWeight::Higher) {
        v[HYDRATION] += 0.10;
        v[ENDURANCE] += 0.05;
    }

    // Already covered by a multi → do not keep asking the model for more
    // micronutrient "wellness" hits.
    let existing = existing_stack(p);
    if existing.multivitamin {
        v[WELLNESS] *= 0.35;
    }
    if existing.vitamin_d {
        v[WELLNESS] *= 0.85;
    }

    // Mood is never a declared goal in this take-home set — keep it quiet
    // so DLPA does not sneak in on a weak cosine.
    if !p.has_goal(Goal::Stress) {
        v[MOOD] = v[MOOD].min(0.15);
    }

    clamp_unit(&mut v);
    v
}

pub fn all_personas() -> Vec<Persona> {
    vec![persona_a(), persona_b(), persona_c(), persona_k()]
}

pub fn persona_by_id(id: &str) -> Option<Persona> {
    all_personas()
        .into_iter()
        .find(|p| p.id.eq_ignore_ascii_case(id))
}

pub fn persona_a() -> Persona {
    Persona {
        id: "A",
        label: "Persona A — perimenopause, energy + sleep",
        summary: "45-54, female, omnivore, marketing, standard hours. Sleeps 7–8h, wakes 2–4am, sometimes hot. Trains 3x/week, mixed. 2 coffees, last one early afternoon. Cycles becoming irregular, periods heavy and getting heavier. No medications. Takes a multivitamin. Goals: energy, sleep.",
        age_band: AgeBand::A45_54,
        sex: Sex::Female,
        diet: Diet::Omnivore,
        occupation: "marketing",
        work: WorkPattern::StandardHours,
        sleep: SleepProfile {
            duration: "7–8h",
            night_waking: true,
            night_waking_detail: Some("wakes 2–4am"),
            unrefreshed: false,
            snores: false,
            witnessed_apneas: false,
            wired_at_bedtime: false,
            vasomotor_symptoms: true,
            occasional_early_waking: false,
        },
        training: Training::MixedGym {
            sessions_per_week: 3,
        },
        caffeine: CaffeineIntake {
            cups: 2,
            last: CaffeineLast::EarlyAfternoon,
        },
        hormones: HormoneStatus {
            cycles_irregular: true,
            periods_heavy: true,
            perimenopause: true,
            post_menopause: false,
            on_hrt: false,
        },
        medications: vec![],
        existing_supplements: vec!["multivitamin".into()],
        allergies: vec![],
        conditions: vec![],
        goals: vec![Goal::Energy, Goal::Sleep],
        body_weight: BodyWeight::Typical,
    }
}

pub fn persona_b() -> Persona {
    Persona {
        id: "B",
        label: "Persona B — unrefreshing sleep, possible OSA, energy + sleep",
        summary: "45-54, male, omnivore, field sales, long hours driving. Sleeps 8h+ and wakes unrefreshed, snores, partner has seen him stop breathing. No training. 3 coffees, last one mid-afternoon. Higher body weight, no diagnosed conditions. No medications. Takes nothing. Goals: sleep, energy.",
        age_band: AgeBand::A45_54,
        sex: Sex::Male,
        diet: Diet::Omnivore,
        occupation: "field sales",
        work: WorkPattern::LongHoursDriving,
        sleep: SleepProfile {
            duration: "8h+",
            night_waking: false,
            night_waking_detail: None,
            unrefreshed: true,
            snores: true,
            witnessed_apneas: true,
            wired_at_bedtime: false,
            vasomotor_symptoms: false,
            occasional_early_waking: false,
        },
        training: Training::None,
        caffeine: CaffeineIntake {
            cups: 3,
            last: CaffeineLast::MidAfternoon,
        },
        hormones: HormoneStatus {
            cycles_irregular: false,
            periods_heavy: false,
            perimenopause: false,
            post_menopause: false,
            on_hrt: false,
        },
        medications: vec![],
        existing_supplements: vec![],
        allergies: vec![],
        conditions: vec!["possible obstructive sleep apnea (snoring + witnessed apneas + unrefreshing sleep)".into()],
        goals: vec![Goal::Sleep, Goal::Energy],
        body_weight: BodyWeight::Higher,
    }
}

pub fn persona_c() -> Persona {
    Persona {
        id: "C",
        label: "Persona C — wired startup ops, energy + stress",
        summary: "25-34, male, omnivore, startup operations, long hours. Sleeps under 6h, wired at bedtime. Mostly seated, no training. 4 coffees, last one late afternoon. Bloods normal last year. No medications. Takes nothing. Goals: energy, stress.",
        age_band: AgeBand::A25_34,
        sex: Sex::Male,
        diet: Diet::Omnivore,
        occupation: "startup operations",
        work: WorkPattern::LongHoursSeated,
        sleep: SleepProfile {
            duration: "<6h",
            night_waking: false,
            night_waking_detail: None,
            unrefreshed: false,
            snores: false,
            witnessed_apneas: false,
            wired_at_bedtime: true,
            vasomotor_symptoms: false,
            occasional_early_waking: false,
        },
        training: Training::None,
        caffeine: CaffeineIntake {
            cups: 4,
            last: CaffeineLast::LateAfternoon,
        },
        hormones: HormoneStatus {
            cycles_irregular: false,
            periods_heavy: false,
            perimenopause: false,
            post_menopause: false,
            on_hrt: false,
        },
        medications: vec![],
        existing_supplements: vec![],
        allergies: vec![],
        conditions: vec![],
        goals: vec![Goal::Energy, Goal::Stress],
        body_weight: BodyWeight::Typical,
    }
}

pub fn persona_k() -> Persona {
    Persona {
        id: "K",
        label: "Persona K — post-menopause on HRT, healthy ageing + joints",
        summary: "55-64, female, pescatarian, part-time consultant, standard hours. Sleeps 7–8h, occasional early waking. Yoga and walking 4x/week. 1 coffee, morning. Post-menopause, on HRT. Takes vitamin D and omega-3. Goals: healthy ageing, joint comfort.",
        age_band: AgeBand::A55_64,
        sex: Sex::Female,
        diet: Diet::Pescatarian,
        occupation: "part-time consultant",
        work: WorkPattern::StandardHours,
        sleep: SleepProfile {
            duration: "7–8h",
            night_waking: false,
            night_waking_detail: None,
            unrefreshed: false,
            snores: false,
            witnessed_apneas: false,
            wired_at_bedtime: false,
            vasomotor_symptoms: false,
            occasional_early_waking: true,
        },
        training: Training::YogaWalking {
            sessions_per_week: 4,
        },
        caffeine: CaffeineIntake {
            cups: 1,
            last: CaffeineLast::Morning,
        },
        hormones: HormoneStatus {
            cycles_irregular: false,
            periods_heavy: false,
            perimenopause: false,
            post_menopause: true,
            on_hrt: true,
        },
        medications: vec!["HRT".into()],
        existing_supplements: vec!["vitamin D".into(), "omega-3".into()],
        allergies: vec![],
        conditions: vec![],
        goals: vec![Goal::HealthyAgeing, Goal::JointComfort],
        body_weight: BodyWeight::Typical,
    }
}

pub fn vector_preview(p: &Persona) -> Vec<(String, f32)> {
    features::nonzero_dims(&vectorize(p), 0.20)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::cosine;

    #[test]
    fn four_personas_present() {
        assert_eq!(all_personas().len(), 4);
        assert!(persona_by_id("a").is_some());
        assert!(persona_by_id("K").is_some());
    }

    #[test]
    fn sleep_goal_suppresses_stim_need() {
        let v = vectorize(&persona_a());
        assert!(v[ENERGY_STIM] < 0.15);
        assert!(v[SLEEP_MAINT] > 0.6);
    }

    #[test]
    fn persona_vectors_are_not_identical() {
        let a = vectorize(&persona_a());
        let k = vectorize(&persona_k());
        assert!(cosine(&a, &k) < 0.85);
    }

    #[test]
    fn existing_multi_flags_typical_micros() {
        let e = existing_stack(&persona_a());
        assert!(e.multivitamin && e.vitamin_d && e.zinc && e.b12);
    }

    #[test]
    fn persona_k_flags_d_and_omega3_and_hrt() {
        let p = persona_k();
        let e = existing_stack(&p);
        assert!(e.vitamin_d && e.omega3 && !e.multivitamin);
        assert!(p.on_hrt());
    }
}
