//! Exact CLD-9 public custom-builder catalog (`initialIngredients` on
//! https://www.cld9.ai/custom). Only these 20 actives — and only their
//! offered doses — may be recommended. Excipients / flavors are not actives.

use crate::features::{self, FeatureVec};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Cognitive,
    Endurance,
    Energy,
    Focus,
    Hydration,
    Immunity,
    Mood,
    Performance,
    Power,
    Recovery,
    Wellness,
}

impl Category {
    pub fn as_str(self) -> &'static str {
        match self {
            Category::Cognitive => "Cognitive",
            Category::Endurance => "Endurance",
            Category::Energy => "Energy",
            Category::Focus => "Focus",
            Category::Hydration => "Hydration",
            Category::Immunity => "Immunity",
            Category::Mood => "Mood",
            Category::Performance => "Performance",
            Category::Power => "Power",
            Category::Recovery => "Recovery",
            Category::Wellness => "Wellness",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Amino,
    Adaptogen,
    Stimulant,
    Botanical,
    Mineral,
    Vitamin,
    Electrolyte,
    Nootropic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dose {
    pub amount: u32,
    pub unit: &'static str,
}

impl Dose {
    pub const fn mg(amount: u32) -> Self {
        Self { amount, unit: "mg" }
    }
    pub const fn mcg(amount: u32) -> Self {
        Self {
            amount,
            unit: "mcg",
        }
    }
    pub const fn iu(amount: u32) -> Self {
        Self { amount, unit: "IU" }
    }

    pub fn display(self) -> String {
        format!("{}{}", self.amount, self.unit)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStrength {
    Strong,
    Moderate,
    Limited,
    Insufficient,
    Mixed,
}

impl EvidenceStrength {
    pub fn as_str(self) -> &'static str {
        match self {
            EvidenceStrength::Strong => "Strong",
            EvidenceStrength::Moderate => "Moderate",
            EvidenceStrength::Limited => "Limited",
            EvidenceStrength::Insufficient => "Insufficient",
            EvidenceStrength::Mixed => "Mixed",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ingredient {
    pub id: &'static str,
    pub name: &'static str,
    pub alias: Option<&'static str>,
    pub category: Category,
    pub family: Family,
    pub doses: &'static [Dose],
    pub site_blurb: &'static str,
    pub benefit: FeatureVec,
    /// Tokens used for allergy / "already taking" matching.
    pub allergen_keys: &'static [&'static str],
    pub contains_caffeine: bool,
    pub stimulant: bool,
    pub bulky: bool,
    pub evidence: EvidenceStrength,
    pub evidence_note: &'static str,
    pub evidence_sources: &'static str,
}

pub const SACHET: &str = "1 packet daily, 30 servings";
pub const FLAVORS: &str = "Orange Popsicle or Unflavored";
pub const OTHER_INGREDIENTS: &str = "Natural flavors, stevia, citric acid, malic acid, sucralose, ace sulfame potassium (less than .01%), silicon dioxide, magnesium stearate, calcium silicate, maltodextrin (less than .01%)";

pub fn catalog() -> &'static [Ingredient] {
    CATALOG
}

pub fn by_id(id: &str) -> Option<&'static Ingredient> {
    CATALOG.iter().find(|i| i.id == id)
}

const CATALOG: &[Ingredient] = &[
    Ingredient {
        id: "alcar",
        name: "Acetyl-L-Carnitine HCL",
        alias: Some("ALCAR"),
        category: Category::Energy,
        family: Family::Amino,
        doses: &[Dose::mg(500), Dose::mg(1000)],
        site_blurb: "mitochondrial energy / fatty acid metabolism / cognitive support",
        benefit: feat([
            0.85, 0.00, 0.00, 0.00, 0.00, 0.05, 0.15, 0.70, 0.20, 0.10, 0.25, 0.00, 0.55, 0.00,
            0.30, 0.15, 0.20, 0.00,
        ]),
        allergen_keys: &["alcar", "acetyl-l-carnitine", "acetyl l carnitine", "carnitine"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "Meta-analyses suggest a modest benefit for mental/physical fatigue and age-related cognitive complaints; not a stimulant. Evidence is better for older or fatigued adults than for healthy 20-somethings.",
        evidence_sources: "NIH ODS (carnitine fact sheet); Montgomery et al. systematic reviews on ALCAR and cognition/fatigue",
    },
    Ingredient {
        id: "alpha_gpc",
        name: "Alpha-GPC",
        alias: Some("L-alpha-glycerylphosphorylcholine"),
        category: Category::Cognitive,
        family: Family::Nootropic,
        doses: &[Dose::mg(150), Dose::mg(300)],
        site_blurb: "acetylcholine / cognition / performance",
        benefit: feat([
            0.25, 0.00, 0.00, 0.00, 0.00, 0.00, 0.45, 0.90, 0.45, 0.40, 0.10, 0.00, 0.45, 0.00,
            0.15, 0.10, 0.10, 0.15,
        ]),
        allergen_keys: &["alpha-gpc", "alpha gpc", "alphagpc", "choline"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Limited,
        evidence_note: "Used as a choline donor; small trials in sport (acute power) and cognition. Human evidence is thinner than marketing implies. 150–300 mg is a conservative nootropic range vs. the 300–600 mg sport doses in some studies.",
        evidence_sources: "Examine.com Alpha-GPC; limited RCTs on acute power output and cognitive performance",
    },
    Ingredient {
        id: "ashwagandha",
        name: "Ashwagandha Root Extract",
        alias: Some("KSM-66"),
        category: Category::Recovery,
        family: Family::Adaptogen,
        doses: &[Dose::mg(200), Dose::mg(400)],
        site_blurb: "adaptogen stress/recovery",
        benefit: feat([
            0.30, 0.00, 0.55, 0.35, 0.75, 0.95, 0.50, 0.15, 0.10, 0.10, 0.10, 0.00, 0.20, 0.00,
            0.80, 0.15, 0.40, 0.00,
        ]),
        allergen_keys: &[
            "ashwagandha",
            "ksm-66",
            "ksm66",
            "withania",
            "withania somnifera",
        ],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "Several RCTs (often KSM-66 300–600 mg) show reductions in perceived stress/cortisol and some sleep-quality benefit. Not a treatment for mood disorders, thyroid disease, or hormonal conditions. Avoid in pregnancy.",
        evidence_sources: "Lopresti et al. 2019 (KSM-66 stress RCT); Cheah et al. 2021 sleep systematic review; NIH ODS ashwagandha",
    },
    Ingredient {
        id: "boron",
        name: "Boron (as Boron Citrate)",
        alias: None,
        category: Category::Performance,
        family: Family::Mineral,
        doses: &[Dose::mg(10)],
        site_blurb: "bone / mineral metabolism",
        benefit: feat([
            0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.15, 0.10, 0.00, 0.00, 0.50, 0.75,
            0.10, 0.35, 0.00, 0.00,
        ]),
        allergen_keys: &["boron", "boron citrate"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Limited,
        evidence_note: "Boron is involved in bone and sex-hormone metabolism; supplemental evidence is small and inconsistent. 10 mg is a high supplemental dose (typical study doses 3–6 mg). Extra caution when hormones are medically managed (HRT).",
        evidence_sources: "NIH ODS boron fact sheet; small trials on bone/hormone markers (Pizzorno 2015 review)",
    },
    Ingredient {
        id: "caffeine",
        name: "Caffeine",
        alias: None,
        category: Category::Energy,
        family: Family::Stimulant,
        doses: &[Dose::mg(100), Dose::mg(200), Dose::mg(300)],
        site_blurb: "alertness",
        benefit: feat([
            0.15, 1.00, 0.00, 0.00, 0.00, 0.00, 0.20, 0.50, 0.40, 0.20, 0.35, 0.00, 0.00, 0.00,
            0.00, 0.00, 0.15, 0.00,
        ]),
        allergen_keys: &["caffeine", "coffee", "stimulant"],
        contains_caffeine: true,
        stimulant: true,
        bulky: false,
        evidence: EvidenceStrength::Strong,
        evidence_note: "Robust acute evidence for alertness and some exercise performance. Also a reliable sleep disruptor, especially afternoon/evening. Adding 100–300 mg on top of multiple daily coffees is usually the wrong move when sleep is a goal.",
        evidence_sources: "ISSN caffeine position stand; FDA caffeine guidance; Clark & Landolt 2017 (caffeine and sleep)",
    },
    Ingredient {
        id: "cordyceps",
        name: "Cordyceps Extract",
        alias: None,
        category: Category::Energy,
        family: Family::Botanical,
        doses: &[Dose::mg(2000)],
        site_blurb: "ATP / physical energy",
        benefit: feat([
            0.80, 0.00, 0.00, 0.00, 0.00, 0.10, 0.10, 0.10, 0.55, 0.20, 0.75, 0.00, 0.15, 0.00,
            0.35, 0.15, 0.00, 0.20,
        ]),
        allergen_keys: &["cordyceps", "cordyceps militaris", "cordyceps sinensis"],
        contains_caffeine: false,
        stimulant: false,
        bulky: true,
        evidence: EvidenceStrength::Limited,
        evidence_note: "Traditional use plus small human trials on VO2 / fatigue; results are mixed and often in specific extracts. Reasonable as a non-stimulant energy option, not a substitute for sleep or medical evaluation of fatigue.",
        evidence_sources: "Hirsch et al. 2017 cordyceps exercise RCT; Examine.com cordyceps (limited/mixed)",
    },
    Ingredient {
        id: "creatine",
        name: "Creatine (Creatine Monohydrate)",
        alias: None,
        category: Category::Power,
        family: Family::Amino,
        doses: &[Dose::mg(5000)],
        site_blurb: "strength/power",
        benefit: feat([
            0.25, 0.00, 0.00, 0.00, 0.00, 0.00, 0.10, 0.40, 0.80, 0.95, 0.30, 0.00, 0.80, 0.35,
            0.55, 0.20, 0.10, 0.00,
        ]),
        allergen_keys: &["creatine", "creatine monohydrate"],
        contains_caffeine: false,
        stimulant: false,
        bulky: true,
        evidence: EvidenceStrength::Strong,
        evidence_note: "One of the best-supported supplements for strength, high-intensity performance, and lean mass. Growing evidence for older adults (sarcopenia) and a smaller cognitive literature. 5 g/day monohydrate is the standard maintenance dose. Does not treat sleep disorders.",
        evidence_sources: "ISSN creatine position stand (Kreider et al. 2017); Chilibeck et al. ageing/muscle reviews",
    },
    Ingredient {
        id: "dlpa",
        name: "DL-Phenylalanine",
        alias: Some("DLPA"),
        category: Category::Mood,
        family: Family::Amino,
        doses: &[Dose::mg(1000)],
        site_blurb: "mood/dopamine/energy",
        benefit: feat([
            0.45, 0.10, 0.00, 0.00, 0.00, 0.20, 0.15, 0.20, 0.10, 0.00, 0.00, 0.00, 0.00, 0.00,
            0.00, 0.10, 0.70, 0.00,
        ]),
        allergen_keys: &[
            "dl-phenylalanine",
            "dlpa",
            "phenylalanine",
            "phenylalanine",
        ],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Insufficient,
        evidence_note: "Phenylalanine is a precursor to tyrosine/dopamine/catecholamines, but clinical mood evidence for DLPA is weak. Contraindicated with MAOIs and in PKU. We rarely rank this into a sachet unless mood is a primary, otherwise-unmet goal.",
        evidence_sources: "NIH/NLM phenylalanine; sparse older DLPA pain/mood literature — not guideline-supported",
    },
    Ingredient {
        id: "ginkgo",
        name: "Ginkgo Biloba Extract",
        alias: None,
        category: Category::Cognitive,
        family: Family::Botanical,
        doses: &[Dose::mg(120)],
        site_blurb: "cognition / circulation",
        benefit: feat([
            0.10, 0.00, 0.00, 0.00, 0.00, 0.00, 0.20, 0.70, 0.10, 0.00, 0.00, 0.00, 0.50, 0.00,
            0.00, 0.15, 0.10, 0.90,
        ]),
        allergen_keys: &["ginkgo", "ginkgo biloba", "ginko"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Mixed,
        evidence_note: "Cochrane-style reviews in dementia/cognitive decline are mixed; benefit for healthy adults is unproven. Antiplatelet effect → hard block with anticoagulants/blood thinners. Not for OSA-related unrefreshing sleep.",
        evidence_sources: "Birks & Grimley Evans Cochrane review (ginkgo and dementia); NIH ODS ginkgo; bleeding-risk case reports",
    },
    Ingredient {
        id: "green_tea",
        name: "Green Tea Extract (50% EGCG)",
        alias: Some("EGCG"),
        category: Category::Energy,
        family: Family::Botanical,
        doses: &[Dose::mg(250), Dose::mg(500)],
        site_blurb: "antioxidant / energy",
        benefit: feat([
            0.30, 0.55, 0.00, 0.00, 0.00, 0.15, 0.20, 0.20, 0.20, 0.00, 0.25, 0.00, 0.45, 0.00,
            0.10, 0.50, 0.10, 0.15,
        ]),
        allergen_keys: &["green tea", "green tea extract", "egcg", "camellia", "caffeine"],
        contains_caffeine: true,
        stimulant: true,
        bulky: false,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "EGCG has antioxidant data; energy effect is partly residual caffeine. High-dose green tea extracts have rare hepatotoxicity signals — we stay at the lower 250 mg option when used at all. Counts as a caffeine source for allergy and sleep rules.",
        evidence_sources: "NIH ODS green tea; EFSA/FDA notes on green tea extract and liver injury; EGCG caffeine content varies by extract",
    },
    Ingredient {
        id: "himalayan_salt",
        name: "Himalayan Pink Salt",
        alias: Some("electrolytes"),
        category: Category::Hydration,
        family: Family::Electrolyte,
        doses: &[Dose::mg(200), Dose::mg(400)],
        site_blurb: "electrolytes",
        benefit: feat([
            0.05, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.15, 0.00, 0.30, 0.95, 0.00, 0.00,
            0.15, 0.20, 0.00, 0.00,
        ]),
        allergen_keys: &["himalayan", "pink salt", "salt", "sodium", "electrolyte"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Limited,
        evidence_note: "Sodium is a real electrolyte; the 'Himalayan' branding is not magically superior to other salt. Useful as a small sodium bump for heavy coffee users / long driving days. 200–400 mg salt is a modest sodium dose, not a sports-drink replacement.",
        evidence_sources: "NIH ODS sodium; ACSM fluid/electrolyte position stand — pink salt specifically is marketing, not a distinct evidence base",
    },
    Ingredient {
        id: "citrulline",
        name: "L-Citrulline",
        alias: None,
        category: Category::Performance,
        family: Family::Amino,
        doses: &[Dose::mg(3000), Dose::mg(6000)],
        site_blurb: "blood flow / exercise",
        benefit: feat([
            0.20, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.90, 0.25, 0.65, 0.15, 0.25, 0.20,
            0.30, 0.10, 0.00, 0.85,
        ]),
        allergen_keys: &["citrulline", "l-citrulline", "l citrulline"],
        contains_caffeine: false,
        stimulant: false,
        bulky: true,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "Raises arginine/NO; most performance trials use 6–8 g citrulline (or ~3–5 g citrulline malate, not identical). We use 3 g for non-lifters and 6 g only when there is a real training stimulus. Not a joint-pain drug.",
        evidence_sources: "Gonzalez & Trexler 2020 citrulline review; Examine.com L-citrulline",
    },
    Ingredient {
        id: "theanine",
        name: "L-Theanine",
        alias: None,
        category: Category::Focus,
        family: Family::Amino,
        doses: &[Dose::mg(200), Dose::mg(400)],
        site_blurb: "calm focus",
        benefit: feat([
            0.05, 0.00, 0.70, 0.35, 0.65, 0.70, 0.95, 0.40, 0.05, 0.00, 0.00, 0.00, 0.10, 0.00,
            0.20, 0.10, 0.30, 0.00,
        ]),
        allergen_keys: &["l-theanine", "theanine", "l theanine"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "Consistent small-to-moderate effects on relaxation and the caffeine 'edge' (classic combo). Sleep data are promising but not definitive. Very good fit when coffee intake is already high and the goal includes sleep or stress.",
        evidence_sources: "Owen et al. 2008 / Einöther & Giesbrecht 2013 caffeine+theanine; Hidese et al. 2019 stress/sleep RCT",
    },
    Ingredient {
        id: "magnesium",
        name: "Magnesium (as Magnesium Malate)",
        alias: None,
        category: Category::Recovery,
        family: Family::Mineral,
        doses: &[Dose::mg(200), Dose::mg(400)],
        site_blurb: "muscle/nerve (site lists malate; many sleep stacks use glycinate)",
        benefit: feat([
            0.15, 0.00, 0.55, 0.90, 0.80, 0.50, 0.40, 0.10, 0.20, 0.15, 0.15, 0.10, 0.35, 0.40,
            0.85, 0.45, 0.15, 0.00,
        ]),
        allergen_keys: &["magnesium", "magnesium malate", "malate"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "Magnesium is essential for nerve/muscle function; deficiency is common enough that repletion can help cramps, sleep quality, and recovery. Sleep RCTs are mixed (better if low baseline). Catalog salt is malate (more often daytime/energy) rather than glycinate — we still use it because it is what CLD-9 offers, and note the form quirk.",
        evidence_sources: "NIH ODS magnesium; Abbasi et al. 2012 insomnia RCT; 2021–2024 reviews = mixed/moderate for sleep",
    },
    Ingredient {
        id: "taurine",
        name: "Taurine",
        alias: None,
        category: Category::Endurance,
        family: Family::Amino,
        doses: &[Dose::mg(1000), Dose::mg(2000)],
        site_blurb: "endurance/hydration",
        benefit: feat([
            0.30, 0.00, 0.20, 0.15, 0.40, 0.20, 0.25, 0.10, 0.35, 0.10, 0.75, 0.60, 0.40, 0.10,
            0.50, 0.20, 0.15, 0.15,
        ]),
        allergen_keys: &["taurine"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "Endurance meta-analyses support a small performance effect (~1 g+). Also an osmolyte (hydration pairing). Preliminary data on blood pressure and sleep are not practice-changing. Safe at 1–2 g for healthy adults.",
        evidence_sources: "Waldron et al. 2018 taurine endurance meta-analysis; NIH/Examine taurine safety notes",
    },
    Ingredient {
        id: "b12",
        name: "Vitamin B12 (Methylcobalamin)",
        alias: None,
        category: Category::Wellness,
        family: Family::Vitamin,
        doses: &[Dose::mcg(200)],
        site_blurb: "wellness / energy metabolism",
        benefit: feat([
            0.50, 0.00, 0.00, 0.00, 0.00, 0.00, 0.10, 0.20, 0.00, 0.00, 0.00, 0.00, 0.25, 0.00,
            0.00, 0.75, 0.15, 0.00,
        ]),
        allergen_keys: &["b12", "b-12", "vitamin b12", "methylcobalamin", "cobalamin"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Strong,
        evidence_note: "Corrects deficiency (vegans, some older adults, metformin users). Does not boost energy in people who are replete — Persona C's recent normal bloods argue against stacking it for 'energy'. 200 mcg methylcobalamin is a modest supplemental dose.",
        evidence_sources: "NIH ODS vitamin B12; Cochrane B12 deficiency reviews",
    },
    Ingredient {
        id: "b6",
        name: "Vitamin B6 (as P5P)",
        alias: Some("pyridoxal-5-phosphate"),
        category: Category::Energy,
        family: Family::Vitamin,
        doses: &[Dose::mg(20)],
        site_blurb: "energy metabolism",
        benefit: feat([
            0.45, 0.00, 0.00, 0.00, 0.00, 0.15, 0.10, 0.10, 0.00, 0.00, 0.00, 0.00, 0.10, 0.00,
            0.00, 0.60, 0.40, 0.00,
        ]),
        allergen_keys: &["b6", "b-6", "vitamin b6", "p5p", "pyridoxal", "pyridoxine"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "Required cofactor in neurotransmitter/energy metabolism. Extra B6 does not equal extra energy if intake is adequate. 20 mg P5P is well below the 100 mg/day UL but is already a high multiple of the RDA — do not stack on a multi without a reason.",
        evidence_sources: "NIH ODS vitamin B6 (RDA 1.3–1.7 mg; UL 100 mg); neuropathy risk is mainly chronic very high doses",
    },
    Ingredient {
        id: "vitamin_c",
        name: "Vitamin C (Ascorbic Acid)",
        alias: None,
        category: Category::Immunity,
        family: Family::Vitamin,
        doses: &[Dose::mg(100)],
        site_blurb: "immunity / collagen cofactor",
        benefit: feat([
            0.10, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.35, 0.50,
            0.15, 0.55, 0.00, 0.00,
        ]),
        allergen_keys: &["vitamin c", "ascorbic", "ascorbate"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Moderate,
        evidence_note: "Essential for collagen cross-linking (joint/skin) and as an antioxidant. 100 mg covers the RDA-ish range but is not a megadose immune protocol. Cochrane: regular vitamin C does not prevent colds in the general population; it may slightly shorten duration.",
        evidence_sources: "NIH ODS vitamin C; Hemilä & Chalker Cochrane review (colds); collagen-synthesis biochemistry",
    },
    Ingredient {
        id: "vitamin_d",
        name: "Vitamin D (Cholecalciferol)",
        alias: Some("D3"),
        category: Category::Wellness,
        family: Family::Vitamin,
        doses: &[Dose::iu(4000)],
        site_blurb: "wellness / bone",
        benefit: feat([
            0.15, 0.00, 0.10, 0.00, 0.15, 0.00, 0.00, 0.00, 0.00, 0.10, 0.00, 0.00, 0.65, 0.70,
            0.10, 0.80, 0.15, 0.00,
        ]),
        allergen_keys: &["vitamin d", "vit d", "d3", "cholecalciferol"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Strong,
        evidence_note: "Critical for bone/mineral homeostasis; repletion matters if deficient. 4000 IU is the IOM UL for adults — a real dose, not a dusting. Blindly stacking another 4000 IU on top of an existing D supplement (or a multi that already contains D) is a safety miss. We do not treat 'low energy' with D without a level.",
        evidence_sources: "NIH ODS vitamin D; IOM/NAM UL 4000 IU; Endocrine Society deficiency guidance",
    },
    Ingredient {
        id: "zinc",
        name: "Zinc (as Zinc Glycinate)",
        alias: None,
        category: Category::Wellness,
        family: Family::Mineral,
        doses: &[Dose::mg(10)],
        site_blurb: "wellness",
        benefit: feat([
            0.10, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00, 0.40, 0.15,
            0.25, 0.70, 0.10, 0.00,
        ]),
        allergen_keys: &["zinc", "zinc glycinate"],
        contains_caffeine: false,
        stimulant: false,
        bulky: false,
        evidence: EvidenceStrength::Strong,
        evidence_note: "Essential mineral; deficiency impairs immunity and healing. 10 mg is a modest add-on (RDA 8–11 mg). Multivitamins often already contain 8–15 mg; stacking can crowd the UL (40 mg) over time and compete with copper. Food-first for pescatarians/omnivores.",
        evidence_sources: "NIH ODS zinc; immune-function reviews (Wessels et al.)",
    },
];

const fn feat(v: [f32; features::DIM]) -> FeatureVec {
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exactly_twenty_actives() {
        assert_eq!(CATALOG.len(), 20);
    }

    #[test]
    fn ids_unique() {
        let mut ids: Vec<_> = CATALOG.iter().map(|i| i.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 20);
    }
}
