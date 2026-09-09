# Persona C — wired startup ops, energy + stress

**NOT MEDICAL ADVICE. Educational take-home only. Not a diagnosis, prescription, or substitute for a clinician.**

Daily sachet stack of **6 actives** (CLD-9 custom library + offered doses only). Goals: **energy, stress**.

> 25-34, male, omnivore, startup operations, long hours. Sleeps under 6h, wired at bedtime. Mostly seated, no training. 4 coffees, last one late afternoon. Bloods normal last year. No medications. Takes nothing. Goals: energy, stress.

## Feature vector

Quiz/persona answers encoded onto the shared 18-dim benefit space (dims ≥ 0.20 shown).

| Dimension | Weight |
|---|---:|
| `energy_nonstim` | 0.85 |
| `sleep_onset` | 0.80 |
| `stress` | 1.00 |
| `calm_focus` | 1.00 |
| `cognition` | 0.20 |
| `endurance` | 0.25 |
| `recovery` | 0.20 |

## Safety / clinical flags

- Not medical advice. Educational take-home only — not a diagnosis, prescription, or care plan.
- Caffeine context: 4 coffee(s)/day, last dose late afternoon. Stimulants (caffeine powder, green tea extract) are blocked or heavily penalized; L-theanine / magnesium are preferred.
- Wired at bedtime + late caffeine: first intervention is timing (no coffee after early afternoon), not another capsule. The sachet can only support that.

## Ranked daily-sachet stack

### 1. L-Theanine — 400mg (Focus)

- **Score** 0.87 · cosine 0.76 · goal bonus 0.85 · combo +0.08
- **Timing:** evening / with last meal (wind-down)
- **Why it fits:** Goal fit (energy, stress): L-theanine covers calm-focus and sleep-onset without adding stimulant load. Cosine 0.76. Already drinking 4 coffee(s), last dose late afternoon — this is the standard pairing so existing caffeine is less jagged, not a reason to add more caffeine.
- **Evidence (Moderate):** Consistent small-to-moderate effects on relaxation and the caffeine 'edge' (classic combo). Sleep data are promising but not definitive. Very good fit when coffee intake is already high and the goal includes sleep or stress. _Owen et al. 2008 / Einöther & Giesbrecht 2013 caffeine+theanine; Hidese et al. 2019 stress/sleep RCT_
- **Safety:** No specific contraindication fired for this persona.

### 2. Ashwagandha Root Extract — 400mg (Recovery)

- **Score** 0.67 · cosine 0.71 · goal bonus 0.57 · combo +0.00
- **Timing:** evening / with last meal (wind-down)
- **Why it fits:** Goal fit (energy, stress): KSM-66-class ashwagandha is the catalog's stress/recovery adaptogen (400mg). Cosine 0.71 against stress + sleep-quality tags. Helpful for perceived stress; it does not treat perimenopause, HRT, or mood disorders.
- **Evidence (Moderate):** Several RCTs (often KSM-66 300–600 mg) show reductions in perceived stress/cortisol and some sleep-quality benefit. Not a treatment for mood disorders, thyroid disease, or hormonal conditions. Avoid in pregnancy. _Lopresti et al. 2019 (KSM-66 stress RCT); Cheah et al. 2021 sleep systematic review; NIH ODS ashwagandha_
- **Safety:** No specific contraindication fired for this persona.

### 3. Acetyl-L-Carnitine HCL — 500mg (Energy)

- **Score** 0.49 · cosine 0.47 · goal bonus 0.55 · combo +0.00
- **Timing:** morning / early day (with food)
- **Why it fits:** Goal fit (energy, stress): Acetyl-L-carnitine 500mg is the non-stim mitochondrial / cognitive option. Cosine 0.47. Useful when the user wants energy but coffee is already doing the catecholamine job.
- **Evidence (Moderate):** Meta-analyses suggest a modest benefit for mental/physical fatigue and age-related cognitive complaints; not a stimulant. Evidence is better for older or fatigued adults than for healthy 20-somethings. _NIH ODS (carnitine fact sheet); Montgomery et al. systematic reviews on ALCAR and cognition/fatigue_
- **Safety:** No specific contraindication fired for this persona.

### 4. Taurine — 2000mg (Endurance)

- **Score** 0.48 · cosine 0.48 · goal bonus 0.48 · combo +0.00
- **Timing:** any consistent daily time; training days with the session if relevant
- **Why it fits:** Goal fit (energy, stress): Taurine 2000mg supports endurance/hydration and is a reasonable sachet partner for high coffee intake or long driving days. Cosine 0.48. Modest evidence; very clean safety at 1–2 g.
- **Evidence (Moderate):** Endurance meta-analyses support a small performance effect (~1 g+). Also an osmolyte (hydration pairing). Preliminary data on blood pressure and sleep are not practice-changing. Safe at 1–2 g for healthy adults. _Waldron et al. 2018 taurine endurance meta-analysis; NIH/Examine taurine safety notes_
- **Safety:** No specific contraindication fired for this persona.

### 5. Magnesium (as Magnesium Malate) — 400mg (Recovery)

- **Score** 0.47 · cosine 0.48 · goal bonus 0.45 · combo +0.00
- **Timing:** evening / with last meal (wind-down)
- **Why it fits:** Goal fit (energy, stress): magnesium is the catalog mineral for nerve/muscle recovery and the most defensible sleep-adjacent pick CLD-9 actually sells. Cosine 0.48 vs sleep-maintenance / recovery / joint dims. Form is malate (site Recovery label) — not glycinate — we do not invent a salt they do not offer. Dose 400mg (offered: 200 or 400 mg); 400 mg because sleep, night waking, wired evenings, or joint comfort is on the card.
- **Evidence (Moderate):** Magnesium is essential for nerve/muscle function; deficiency is common enough that repletion can help cramps, sleep quality, and recovery. Sleep RCTs are mixed (better if low baseline). Catalog salt is malate (more often daytime/energy) rather than glycinate — we still use it because it is what CLD-9 offers, and note the form quirk. _NIH ODS magnesium; Abbasi et al. 2012 insomnia RCT; 2021–2024 reviews = mixed/moderate for sleep_
- **Safety:** No specific contraindication fired for this persona.

### 6. Cordyceps Extract — 2000mg (Energy)

- **Score** 0.41 · cosine 0.47 · goal bonus 0.25 · combo +0.00
- **Timing:** morning / early day (with food)
- **Why it fits:** Goal fit (energy, stress): Cordyceps 2000 mg is CLD-9's non-stim 'ATP / physical energy' pick. Cosine 0.47. Evidence is limited/mixed — included as a stimulant alternative, not as a proven VO2 drug.
- **Evidence (Limited):** Traditional use plus small human trials on VO2 / fatigue; results are mixed and often in specific extracts. Reasonable as a non-stimulant energy option, not a substitute for sleep or medical evaluation of fatigue. _Hirsch et al. 2017 cordyceps exercise RCT; Examine.com cordyceps (limited/mixed)_
- **Safety:** No specific contraindication fired for this persona.

## Combinations

- Combo: L-theanine + the 4 coffee(s) they already drink (not extra sachet caffeine). This is the best-supported pairing in the catalog.
- Combo: magnesium + L-theanine as the sleep/stress core of the packet (mineral + calm-focus amino).
- Delivery: 6 actives in one daily sachet (1 packet daily, 30 servings, Orange Popsicle or Unflavored). Other label ingredients are excipients, not recommendable actives.

## Catalog notes

- Catalog lock: only the 20 actives and listed doses from cld9.ai/custom `initialIngredients`. No off-catalog bottles (no iron, no omega-3, no glycinate swap, no extra herbs).

## Safety filter (blocked / penalized)

| Action | Ingredient | Final | Cosine | Why |
|---|---|---:|---:|---|
| HARD BLOCK | Caffeine | 0.00 | 0.21 | HARD BLOCK — Caffeine blocked: wired-at-bedtime and/or late, high caffeine intake (4 cups, last late afternoon). More stimulant is the opposite intervention. |
| HARD BLOCK | Green Tea Extract (50% EGCG) | 0.00 | 0.38 | HARD BLOCK — Green Tea Extract (50% EGCG) blocked: wired-at-bedtime and/or late, high caffeine intake (4 cups, last late afternoon). More stimulant is the opposite intervention. |

