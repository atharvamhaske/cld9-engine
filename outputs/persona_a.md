# Persona A — perimenopause, energy + sleep

**NOT MEDICAL ADVICE. Educational take-home only. Not a diagnosis, prescription, or substitute for a clinician.**

Daily sachet stack of **7 actives** (CLD-9 custom library + offered doses only). Goals: **energy, sleep**.

> 45-54, female, omnivore, marketing, standard hours. Sleeps 7–8h, wakes 2–4am, sometimes hot. Trains 3x/week, mixed. 2 coffees, last one early afternoon. Cycles becoming irregular, periods heavy and getting heavier. No medications. Takes a multivitamin. Goals: energy, sleep.

## Feature vector

Quiz/persona answers encoded onto the shared 18-dim benefit space (dims ≥ 0.20 shown).

| Dimension | Weight |
|---|---:|
| `energy_nonstim` | 0.85 |
| `sleep_onset` | 0.50 |
| `sleep_maintenance` | 1.00 |
| `sleep_quality` | 0.95 |
| `calm_focus` | 0.45 |
| `cognition` | 0.20 |
| `performance` | 0.35 |
| `power` | 0.35 |
| `endurance` | 0.25 |
| `recovery` | 0.50 |

## Safety / clinical flags

- Not medical advice. Educational take-home only — not a diagnosis, prescription, or care plan.
- PERSONA A / CYCLE FLAG: Cycles becoming irregular and periods heavy / heavier is not something a sachet should 'treat'. See a clinician (PMB/AUB workup, thyroid, ferritin). Iron is not in the CLD-9 catalog — we will not invent it, and we will not assume deficiency on top of a multivitamin.
- Already takes a multivitamin: B6, B12, C, D, and zinc are treated as covered and are hard-blocked or heavily down-ranked so the sachet does not silently double a multi.
- Caffeine context: 2 coffee(s)/day, last dose early afternoon. Stimulants (caffeine powder, green tea extract) are blocked or heavily penalized; L-theanine / magnesium are preferred.

## Ranked daily-sachet stack

### 1. L-Theanine — 400mg (Focus)

- **Score** 0.78 · cosine 0.65 · goal bonus 0.85 · combo +0.08
- **Timing:** evening / with last meal (wind-down)
- **Why it fits:** Goal fit (energy, sleep): L-theanine covers calm-focus and sleep-onset without adding stimulant load. Cosine 0.65. Already drinking 2 coffee(s), last dose early afternoon — this is the standard pairing so existing caffeine is less jagged, not a reason to add more caffeine.
- **Evidence (Moderate):** Consistent small-to-moderate effects on relaxation and the caffeine 'edge' (classic combo). Sleep data are promising but not definitive. Very good fit when coffee intake is already high and the goal includes sleep or stress. _Owen et al. 2008 / Einöther & Giesbrecht 2013 caffeine+theanine; Hidese et al. 2019 stress/sleep RCT_
- **Safety:** No specific contraindication fired for this persona.

### 2. Magnesium (as Magnesium Malate) — 400mg (Recovery)

- **Score** 0.78 · cosine 0.80 · goal bonus 0.57 · combo +0.04
- **Timing:** evening / with last meal (wind-down)
- **Why it fits:** Goal fit (energy, sleep): magnesium is the catalog mineral for nerve/muscle recovery and the most defensible sleep-adjacent pick CLD-9 actually sells. Cosine 0.80 vs sleep-maintenance / recovery / joint dims. Form is malate (site Recovery label) — not glycinate — we do not invent a salt they do not offer. Dose 400mg (offered: 200 or 400 mg); 400 mg because sleep, night waking, wired evenings, or joint comfort is on the card.
- **Evidence (Moderate):** Magnesium is essential for nerve/muscle function; deficiency is common enough that repletion can help cramps, sleep quality, and recovery. Sleep RCTs are mixed (better if low baseline). Catalog salt is malate (more often daytime/energy) rather than glycinate — we still use it because it is what CLD-9 offers, and note the form quirk. _NIH ODS magnesium; Abbasi et al. 2012 insomnia RCT; 2021–2024 reviews = mixed/moderate for sleep_
- **Safety:** No specific contraindication fired for this persona.

### 3. Ashwagandha Root Extract — 200mg (Recovery)

- **Score** 0.68 · cosine 0.72 · goal bonus 0.57 · combo +0.00
- **Timing:** evening / with last meal (wind-down)
- **Why it fits:** Goal fit (energy, sleep): KSM-66-class ashwagandha is the catalog's stress/recovery adaptogen (200mg). Cosine 0.72 against stress + sleep-quality tags. Helpful for perceived stress; it does not treat perimenopause, HRT, or mood disorders.
- **Evidence (Moderate):** Several RCTs (often KSM-66 300–600 mg) show reductions in perceived stress/cortisol and some sleep-quality benefit. Not a treatment for mood disorders, thyroid disease, or hormonal conditions. Avoid in pregnancy. _Lopresti et al. 2019 (KSM-66 stress RCT); Cheah et al. 2021 sleep systematic review; NIH ODS ashwagandha_
- **Safety:** NOTE — Heavy / irregular periods are a clinician question (fibroids, thyroid, iron status). Ashwagandha is not a treatment for menorrhagia.

### 4. Taurine — 1000mg (Endurance)

- **Score** 0.57 · cosine 0.60 · goal bonus 0.50 · combo +0.00
- **Timing:** any consistent daily time; training days with the session if relevant
- **Why it fits:** Goal fit (energy, sleep): Taurine 1000mg supports endurance/hydration and is a reasonable sachet partner for high coffee intake or long driving days. Cosine 0.60. Modest evidence; very clean safety at 1–2 g.
- **Evidence (Moderate):** Endurance meta-analyses support a small performance effect (~1 g+). Also an osmolyte (hydration pairing). Preliminary data on blood pressure and sleep are not practice-changing. Safe at 1–2 g for healthy adults. _Waldron et al. 2018 taurine endurance meta-analysis; NIH/Examine taurine safety notes_
- **Safety:** No specific contraindication fired for this persona.

### 5. Cordyceps Extract — 2000mg (Energy)

- **Score** 0.53 · cosine 0.54 · goal bonus 0.52 · combo +0.00
- **Timing:** morning / early day (with food)
- **Why it fits:** Goal fit (energy, sleep): Cordyceps 2000 mg is CLD-9's non-stim 'ATP / physical energy' pick. Cosine 0.54. Evidence is limited/mixed — included as a stimulant alternative, not as a proven VO2 drug.
- **Evidence (Limited):** Traditional use plus small human trials on VO2 / fatigue; results are mixed and often in specific extracts. Reasonable as a non-stimulant energy option, not a substitute for sleep or medical evaluation of fatigue. _Hirsch et al. 2017 cordyceps exercise RCT; Examine.com cordyceps (limited/mixed)_
- **Safety:** No specific contraindication fired for this persona.

### 6. Acetyl-L-Carnitine HCL — 500mg (Energy)

- **Score** 0.50 · cosine 0.49 · goal bonus 0.55 · combo +0.00
- **Timing:** morning / early day (with food)
- **Why it fits:** Goal fit (energy, sleep): Acetyl-L-carnitine 500mg is the non-stim mitochondrial / cognitive option. Cosine 0.49. Useful when the user wants energy but coffee is already doing the catecholamine job.
- **Evidence (Moderate):** Meta-analyses suggest a modest benefit for mental/physical fatigue and age-related cognitive complaints; not a stimulant. Evidence is better for older or fatigued adults than for healthy 20-somethings. _NIH ODS (carnitine fact sheet); Montgomery et al. systematic reviews on ALCAR and cognition/fatigue_
- **Safety:** No specific contraindication fired for this persona.

### 7. Creatine (Creatine Monohydrate) — 5000mg (Power)

- **Score** 0.45 · cosine 0.39 · goal bonus 0.43 · combo +0.05
- **Timing:** any consistent daily time; training days with the session if relevant
- **Why it fits:** Goal fit (energy, sleep): Creatine monohydrate 5000 mg (only offered dose) is the strongest evidence-backed active in this catalog for power, training, and healthy ageing / sarcopenia. Cosine 0.39. Not a sleep treatment.
- **Evidence (Strong):** One of the best-supported supplements for strength, high-intensity performance, and lean mass. Growing evidence for older adults (sarcopenia) and a smaller cognitive literature. 5 g/day monohydrate is the standard maintenance dose. Does not treat sleep disorders. _ISSN creatine position stand (Kreider et al. 2017); Chilibeck et al. ageing/muscle reviews_
- **Safety:** No specific contraindication fired for this persona.

## Combinations

- Combo: L-theanine + the 2 coffee(s) they already drink (not extra sachet caffeine). This is the best-supported pairing in the catalog.
- Combo: magnesium + L-theanine as the sleep/stress core of the packet (mineral + calm-focus amino).
- Delivery: 7 actives in one daily sachet (1 packet daily, 30 servings, Orange Popsicle or Unflavored). Other label ingredients are excipients, not recommendable actives.

## Catalog notes

- Catalog lock: only the 20 actives and listed doses from cld9.ai/custom `initialIngredients`. No off-catalog bottles (no iron, no omega-3, no glycinate swap, no extra herbs).
- Iron is not a CLD-9 custom-builder active. Heavy periods → clinician + labs, not a guessed iron sachet line.

## Safety filter (blocked / penalized)

| Action | Ingredient | Final | Cosine | Why |
|---|---|---:|---:|---|
| HARD BLOCK | Caffeine | 0.00 | 0.25 | HARD BLOCK — Caffeine blocked: sleep is a primary goal and the user already drinks 2 coffees/day. Adding catalog caffeine (100–300 mg) or caffeinated green tea extract fights the sleep goal. |
| HARD BLOCK | Green Tea Extract (50% EGCG) | 0.00 | 0.29 | HARD BLOCK — Green Tea Extract (50% EGCG) blocked: sleep is a primary goal and the user already drinks 2 coffees/day. Adding catalog caffeine (100–300 mg) or caffeinated green tea extract fights the sleep goal. |
| HARD BLOCK | Vitamin D (Cholecalciferol) | 0.00 | 0.16 | HARD BLOCK — Already taking a multivitamin (typically includes vitamin D). Catalog dose is 4000 IU — the IOM UL. Do not stack another 4000 IU on top without a 25-OH-D level and a plan. |
| STRONG PENALTY | Vitamin B6 (as P5P) | 0.03 | 0.27 | STRONG PENALTY — B6 already covered by the multivitamin. Catalog dose is 20 mg P5P (many times the RDA). No indication to restack. |
| STRONG PENALTY | Vitamin B12 (Methylcobalamin) | 0.03 | 0.27 | STRONG PENALTY — B12 already covered by the multivitamin. Extra methylcobalamin is not an energy drug when intake is adequate. NOTE — Heavy periods: do not assume iron deficiency and do not invent iron (not in the CLD-9 catalog). Ferritin/… |
| STRONG PENALTY | Zinc (as Zinc Glycinate) | 0.02 | 0.13 | STRONG PENALTY — Zinc already covered by the multivitamin. Stacking 10 mg glycinate on a typical multi can be unnecessary and competes with copper over time. |
| STRONG PENALTY | Vitamin C (Ascorbic Acid) | 0.01 | 0.10 | STRONG PENALTY — Vitamin C is already covered (multi or dedicated C). 100 mg more is low-risk but redundant — down-ranked unless a collagen/joint case is unusually strong and the multi is untrusted. NOTE — Heavy periods: do not assume iron… |

