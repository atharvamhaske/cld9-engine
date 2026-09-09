# Persona K — post-menopause on HRT, healthy ageing + joints

**NOT MEDICAL ADVICE. Educational take-home only. Not a diagnosis, prescription, or substitute for a clinician.**

Daily sachet stack of **7 actives** (CLD-9 custom library + offered doses only). Goals: **healthy ageing, joint comfort**.

> 55-64, female, pescatarian, part-time consultant, standard hours. Sleeps 7–8h, occasional early waking. Yoga and walking 4x/week. 1 coffee, morning. Post-menopause, on HRT. Takes vitamin D and omega-3. Goals: healthy ageing, joint comfort.

## Feature vector

Quiz/persona answers encoded onto the shared 18-dim benefit space (dims ≥ 0.20 shown).

| Dimension | Weight |
|---|---:|
| `cognition` | 0.40 |
| `power` | 0.50 |
| `healthy_ageing` | 1.00 |
| `joint_comfort` | 1.00 |
| `recovery` | 0.50 |
| `wellness` | 0.26 |
| `circulation` | 0.50 |

## Safety / clinical flags

- Not medical advice. Educational take-home only — not a diagnosis, prescription, or care plan.
- PERSONA K / HRT FLAG: Hormone replacement is clinician-managed. This engine will not contradict prescribed HRT, will not use ashwagandha or boron as hormone 'support', and will not restack 4000 IU vitamin D on top of an existing D supplement.
- CATALOG GAP: Already taking omega-3. EPA/DHA is not one of the 20 CLD-9 custom-builder actives, so we cannot fold it into the sachet — keep the existing product. This is a feature of their public catalog, not a knock on omega-3.
- Already takes vitamin D: catalog only offers 4000 IU cholecalciferol (IOM UL). We will not add another 4000 IU on top.

## Ranked daily-sachet stack

### 1. Creatine (Creatine Monohydrate) — 5000mg (Power)

- **Score** 0.90 · cosine 0.74 · goal bonus 0.90 · combo +0.11
- **Timing:** any consistent daily time; training days with the session if relevant
- **Why it fits:** Goal fit (healthy ageing, joint comfort): Creatine monohydrate 5000 mg (only offered dose) is the strongest evidence-backed active in this catalog for power, training, and healthy ageing / sarcopenia. Cosine 0.74. Not a sleep treatment.
- **Evidence (Strong):** One of the best-supported supplements for strength, high-intensity performance, and lean mass. Growing evidence for older adults (sarcopenia) and a smaller cognitive literature. 5 g/day monohydrate is the standard maintenance dose. Does not treat sleep disorders. _ISSN creatine position stand (Kreider et al. 2017); Chilibeck et al. ageing/muscle reviews_
- **Safety:** No specific contraindication fired for this persona.

### 2. Vitamin C (Ascorbic Acid) — 100mg (Immunity)

- **Score** 0.72 · cosine 0.73 · goal bonus 0.70 · combo +0.00
- **Timing:** with a meal (CLD-9 directions: 250–300 ml cold water)
- **Why it fits:** Goal fit (healthy ageing, joint comfort): 100 mg ascorbic acid is a collagen-synthesis cofactor and the catalog's Immunity item. Cosine 0.73. Honest evidence: useful as a cofactor, not a joint-pain cure, and not a cold-prevention megadose.
- **Evidence (Moderate):** Essential for collagen cross-linking (joint/skin) and as an antioxidant. 100 mg covers the RDA-ish range but is not a megadose immune protocol. Cochrane: regular vitamin C does not prevent colds in the general population; it may slightly shorten duration. _NIH ODS vitamin C; Hemilä & Chalker Cochrane review (colds); collagen-synthesis biochemistry_
- **Safety:** No specific contraindication fired for this persona.

### 3. Magnesium (as Magnesium Malate) — 400mg (Recovery)

- **Score** 0.57 · cosine 0.48 · goal bonus 0.80 · combo +0.00
- **Timing:** with a meal; evening if stress is the use-case
- **Why it fits:** Goal fit (healthy ageing, joint comfort): magnesium is the catalog mineral for nerve/muscle recovery and the most defensible sleep-adjacent pick CLD-9 actually sells. Cosine 0.48 vs sleep-maintenance / recovery / joint dims. Form is malate (site Recovery label) — not glycinate — we do not invent a salt they do not offer. Dose 400mg (offered: 200 or 400 mg); 400 mg because sleep, night waking, wired evenings, or joint comfort is on the card.
- **Evidence (Moderate):** Magnesium is essential for nerve/muscle function; deficiency is common enough that repletion can help cramps, sleep quality, and recovery. Sleep RCTs are mixed (better if low baseline). Catalog salt is malate (more often daytime/energy) rather than glycinate — we still use it because it is what CLD-9 offers, and note the form quirk. _NIH ODS magnesium; Abbasi et al. 2012 insomnia RCT; 2021–2024 reviews = mixed/moderate for sleep_
- **Safety:** No specific contraindication fired for this persona.

### 4. L-Citrulline — 3000mg (Performance)

- **Score** 0.52 · cosine 0.50 · goal bonus 0.58 · combo +0.00
- **Timing:** any consistent daily time; training days with the session if relevant
- **Why it fits:** Goal fit (healthy ageing, joint comfort): L-citrulline 3000mg is the catalog's blood-flow / exercise amino. Cosine 0.50. Picked at 3 g unless there is a real lifting stimulus (6 g). Joint comfort here is indirect (perfusion), not an anti-inflammatory claim.
- **Evidence (Moderate):** Raises arginine/NO; most performance trials use 6–8 g citrulline (or ~3–5 g citrulline malate, not identical). We use 3 g for non-lifters and 6 g only when there is a real training stimulus. Not a joint-pain drug. _Gonzalez & Trexler 2020 citrulline review; Examine.com L-citrulline_
- **Safety:** No specific contraindication fired for this persona.

### 5. Alpha-GPC — 300mg (Cognitive)

- **Score** 0.50 · cosine 0.55 · goal bonus 0.38 · combo +0.00
- **Timing:** morning / early day (with food)
- **Why it fits:** Goal fit (healthy ageing, joint comfort): Alpha-GPC 300mg is a choline donor for cognition / focus without caffeine. Cosine 0.55. Evidence is limited; used when mental energy is needed and stimulants are off the table.
- **Evidence (Limited):** Used as a choline donor; small trials in sport (acute power) and cognition. Human evidence is thinner than marketing implies. 150–300 mg is a conservative nootropic range vs. the 300–600 mg sport doses in some studies. _Examine.com Alpha-GPC; limited RCTs on acute power output and cognitive performance_
- **Safety:** No specific contraindication fired for this persona.

### 6. Zinc (as Zinc Glycinate) — 10mg (Wellness)

- **Score** 0.49 · cosine 0.57 · goal bonus 0.30 · combo +0.00
- **Timing:** with a meal (CLD-9 directions: 250–300 ml cold water)
- **Why it fits:** Goal fit (healthy ageing, joint comfort): 10 mg zinc glycinate is a modest wellness add. Cosine 0.57. Only belongs if a multi isn't already covering it.
- **Evidence (Strong):** Essential mineral; deficiency impairs immunity and healing. 10 mg is a modest add-on (RDA 8–11 mg). Multivitamins often already contain 8–15 mg; stacking can crowd the UL (40 mg) over time and compete with copper. Food-first for pescatarians/omnivores. _NIH ODS zinc; immune-function reviews (Wessels et al.)_
- **Safety:** No specific contraindication fired for this persona.

### 7. Acetyl-L-Carnitine HCL — 500mg (Energy)

- **Score** 0.47 · cosine 0.47 · goal bonus 0.48 · combo +0.00
- **Timing:** morning / early day (with food)
- **Why it fits:** Goal fit (healthy ageing, joint comfort): Acetyl-L-carnitine 500mg is the non-stim mitochondrial / cognitive option. Cosine 0.47. Better-supported in older / fatigued adults than as a 'pre-workout' — fits healthy ageing cognition.
- **Evidence (Moderate):** Meta-analyses suggest a modest benefit for mental/physical fatigue and age-related cognitive complaints; not a stimulant. Evidence is better for older or fatigued adults than for healthy 20-somethings. _NIH ODS (carnitine fact sheet); Montgomery et al. systematic reviews on ALCAR and cognition/fatigue_
- **Safety:** No specific contraindication fired for this persona.

## Combinations

- Combo: ALCAR + Alpha-GPC as a non-stim cognitive/energy pair (mitochondrial + choline).
- Combo: creatine + L-citrulline (power + blood flow) — only when ageing or training justifies the bulky powders.
- Combo: creatine (muscle/ageing) + vitamin C (collagen cofactor) for the healthy-ageing / joint brief — complementary, not synergistic magic.
- Delivery: 7 actives in one daily sachet (1 packet daily, 30 servings, Orange Popsicle or Unflavored). Other label ingredients are excipients, not recommendable actives.

## Catalog notes

- Catalog lock: only the 20 actives and listed doses from cld9.ai/custom `initialIngredients`. No off-catalog bottles (no iron, no omega-3, no glycinate swap, no extra herbs).
- Omega-3 stays as a separate product — it is not in the CLD-9 custom library.

## Safety filter (blocked / penalized)

| Action | Ingredient | Final | Cosine | Why |
|---|---|---:|---:|---|
| HARD BLOCK | Vitamin D (Cholecalciferol) | 0.00 | 0.74 | HARD BLOCK — Already taking vitamin D. Catalog only offers 4000 IU — stacking that on an existing D supplement is a safety miss, not a wellness upgrade. |
| STRONG PENALTY | Boron (as Boron Citrate) | 0.09 | 0.85 | STRONG PENALTY — On HRT: boron influences estrogen/testosterone metabolism. 10 mg is a high supplemental dose. Not stacked on top of clinician-managed HRT without their OK. |
| STRONG PENALTY | Ashwagandha Root Extract | 0.03 | 0.26 | STRONG PENALTY — On HRT: ashwagandha can nudge thyroid / HPA-axis markers. Do not use it to 'manage' menopause or to second-guess prescribed HRT — clinician-managed hormones win. |

