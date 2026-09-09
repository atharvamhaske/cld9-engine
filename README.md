# CLD-9-style daily-sachet recommendation engine

Take-home: a small, fully runnable **Rust** recommendation engine that mirrors CLD-9’s public product idea — a short quiz, an algorithm that matches answers to **compounds / doses / combinations**, and one **daily drink-mix packet** of **4–8 actives** instead of a cabinet of bottles.

**Not medical advice.** This is an educational assignment. It does not diagnose, treat, or replace a clinician. Persona B’s snoring + witnessed apneas are a medical-referral flag, not a supplement problem.

---

## Quick start (evaluator)

Needs a Rust toolchain (`rustc` / `cargo`). No other setup, no API keys, no extra crates.

```bash
cargo test
cargo run
cargo run --release
cargo run -- --persona A
cargo run -- --persona B
cargo run -- --persona C
cargo run -- --persona K
cargo run -- --persona A --allergy ashwagandha
cargo run -- --persona C --allergy caffeine
cargo run -- --demo-allergy
cargo run -- --write-outputs
```

`cargo run` prints ranked stacks for **A, B, C, K** plus the worked allergy demo.

Sample artifacts (already generated):

- `outputs/persona_a.md` … `outputs/persona_k.md`
- `outputs/allergy_demo.md`
- `outputs/results.md` (all of the above in one file)

---

## Alignment with CLD-9 product

CLD-9’s public site ([cld9.ai](https://www.cld9.ai/), [custom builder](https://www.cld9.ai/custom)) describes three steps:

1. **Quiz** — goals, lifestyle, diet, health, what you already take. No required blood test.
2. **Algorithm** — match answers to the right compounds, doses, and combinations; avoid overlap / waste.
3. **One daily sachet** — a named, pre-measured packet that replaces 6–10 separate bottles. Mix in 250–300 ml cold water. Flavors: Orange Popsicle / Unflavored. 30 servings.

This repo follows that shape on purpose:

| CLD-9 product | This take-home |
|---|---|
| Short quiz → formula | Structured persona/quiz record → 18-dim feature vector |
| Match compounds / doses / combos | Cosine + goal bonus + combo notes, then a **listed catalog dose** |
| Real custom-builder library | **Exact 20 actives + offered doses** from `initialIngredients` on `/custom` |
| One packet vs 6–10 bottles | Ranked **4–8** actives, diversity-capped so the drink stays realistic |
| “No overlap, no waste” | Hard-block / down-rank what they already take (A: multi; K: vitamin D) |
| Research-backed copy | Honest evidence strength + NIH ODS / Cochrane / ISSN / Examine-style citations |
| Safety as a product risk | Allergies, meds, HRT, possible OSA, high caffeine — **first-class**, not a footnote |

**What is simplified vs production**

- No robotics, fulfillment, labeling, or shipping.
- No proprietary model, no chat UI, no monthly “swap the formula” loop.
- Hybrid of **explicit vectors + inspectable rules**, not a black-box ranker. That is the point of a take-home: you can walk the scoring on a whiteboard.
- Evidence notes are literature-level summaries, not CLD-9’s internal evidence graph.
- Magnesium is **malate** because that is what the public builder lists, even though sleep blogs prefer glycinate. We do not invent a form they do not sell.
- Omega-3 and iron are **not** in the public library, so we do not recommend them. We say that out loud (K already takes omega-3; A has heavy periods).

---

## Architecture

```
quiz / persona          catalog item
      │                       │
      ▼                       ▼
18-dim need vector      18-dim benefit vector
      │                       │
      └──────── cosine ───────┘
                    │
           + goal-alignment bonus
           + combo bonus (theanine↔coffee, Mg↔theanine, …)
                    │
              safety layer
         hard block | penalty | note
                    │
        greedy 4–8 assemble
        (1 stimulant max, ≤2 bulky powders, ≤2 vitamins)
                    │
        ranked sachet + explanations
```

### 1. Vectorization

Shared dimensions (see `src/features.rs`):

`energy_nonstim`, `energy_stim`, `sleep_onset`, `sleep_maintenance`, `sleep_quality`, `stress`, `calm_focus`, `cognition`, `performance`, `power`, `endurance`, `hydration`, `healthy_ageing`, `joint_comfort`, `recovery`, `wellness`, `mood`, `circulation`.

`src/persona.rs` `vectorize()` maps quiz fields onto that space. Goals dominate; sleep/caffeine/training/life-stage overlay. A sleep goal or ≥3 coffees drives `energy_stim` toward 0 so the model is not rewarded for pouring caffeine on an already-wired user.

Each of the 20 actives has a hand-set benefit vector in `src/catalog.rs` from its CLD-9 category plus what the evidence actually supports (ginkgo is circulation/cognition, not sleep; B12 is deficiency-not-stimulant; etc.).

### 2. Scoring

```
raw   = 0.72 * cosine(persona, item) + 0.28 * goal_alignment
final = raw * safety_multiplier + combo_bonus
```

`goal_alignment` is a small explicit table so a reviewer can see *why* magnesium beats ginkgo for Persona A’s sleep goal even if both have some cosine.

### 3. Safety layer (this is the demo)

Implemented in `src/safety.rs`. Goal matching **cannot** override a hard block.

| Signal | Action | Example |
|---|---|---|
| Allergy / avoid-list token | **Hard block** | `ashwagandha` drops KSM-66; `caffeine` drops caffeine **and** green tea extract |
| Pregnancy flag | Hard block ashwagandha | modeled even though none of A/B/C/K are pregnant |
| Blood thinners | Hard block ginkgo (and green tea) | antiplatelet / bleeding signal |
| MAOI | Hard block DLPA | catecholamine precursor |
| Possible OSA (B) | Hard block stimulants; clinician banner | supplements do not treat apneas |
| Sleep goal + ≥2 coffees, or late/high caffeine, or wired-at-bedtime | Hard block caffeine + green tea | prefer theanine / magnesium |
| Existing vitamin D (K) or multi that includes D (A) | Hard block 4000 IU D3 | catalog dose **is** the IOM UL |
| Existing multi | Strong penalty / block on B6, B12, C, zinc | no silent double-stack |
| Already taking the same active | Hard block | creatine, magnesium, ashwagandha, theanine |
| HRT (K) | Strong penalty ashwagandha + boron; moderate ginkgo | do not “manage” hormones beside prescribed HRT |
| Heavy / irregular periods (A) | Clinician flag; no invented iron | iron is not in the CLD-9 catalog |

Every recommended line prints **goal fit + evidence note + safety consideration**.

### 4. How to extend the quiz

Add a field on `Persona`, fold it into `vectorize()` (need signal) and/or `safety::evaluate()` (risk signal). New catalog rows must use a dose CLD-9 actually offers — do not invent bottles. Tests in `safety.rs` / `engine.rs` are the regression net for “allergies and meds stay first-class.”

---

## Proposed quiz (and why it changes the stack)

| Question | Why it matters | How it influences recs |
|---|---|---|
| Age band | Sarcopenia, B12 absorption, bone | Creatine / ageing dims up after 55; B12 only if a gap exists |
| Sex | Context for cycle / HRT notes | Does not blindly swap formulas; drives clinician flags |
| Diet | Nutrient-gap prior | Pescatarian + existing omega-3 → catalog-gap note, not a fake EPA line |
| Occupation / hours | Driving, seated stress, missed meals | Hydration pair (taurine + salt) for field sales |
| Sleep duration, night waking, wired, snoring, witnessed apneas | Separates “sleep hygiene + magnesium” from **possible OSA** | Apneas → medical referral, never a “treat OSA” stack |
| Training type / frequency | Justifies creatine / citrulline / bulky powders | No-training + sleep goal ≠ 6 g citrulline |
| Coffee cups + last-dose timing | Strongest everyday safety lever | ≥2–3 cups or late coffee → block sachet caffeine / EGCG; boost theanine |
| Cycle / menopause / HRT | Avoid hormone-active extras | HRT → down-rank ashwagandha & boron |
| Medications | Interaction hard blocks | Blood thinners × ginkgo; MAOI × DLPA |
| Current supplements | Dedup / no waste | Multi covers micros; existing D blocks 4000 IU |
| Allergies / avoid list | Hard exclusions | Worked demo below |
| Goals (1–3) | Primary vector weights | Energy+sleep ≠ energy+stress ≠ ageing+joints |
| Body-weight band | OSA risk context, dosing conservatism | Not a BMI calculator |

---

## Worked allergy example (run this)

```bash
cargo run -- --demo-allergy
# or
cargo run -- --persona A --allergy ashwagandha
cargo run -- --persona C --allergy caffeine
```

**Persona A + `allergies: ["ashwagandha"]`**

- Baseline includes Ashwagandha Root Extract (stress/sleep-quality adaptogen).
- After the flag it is **hard-blocked**. The sachet re-ranks the next valid catalog pick (typically creatine or cordyceps — she already trains 3×/week).
- Magnesium and L-theanine stay. Sleep goal still forbids caffeine / green tea.

**Persona C + `allergies: ["caffeine"]`**

- Even if energy-scoring would have liked a stimulant, caffeine powder **and** green tea extract (residual caffeine + EGCG) are hard-blocked.
- Stack stays theanine / ashwagandha / magnesium / non-stim energy (ALCAR, cordyceps, alpha-GPC).
- This is the point: **safety is not a post-hoc disclaimer**. It changes the packet.

---

## Personas (always printed)

| ID | One-line | What the engine must not get wrong |
|---|---|---|
| **A** | 45–54 F, perimenopause, 2 coffees, trains 3×, multi, energy+sleep | Night waking → Mg/theanine; no extra caffeine; no iron invention; don’t restack the multi |
| **B** | 45–54 M, 8h+ unrefreshed, snores, witnessed apneas, 3 coffees, higher weight | **See a clinician / sleep study.** No “treat OSA.” No stimulants. Adjunct only |
| **C** | 25–34 M, <6h, wired, 4 late coffees, seated, energy+stress | No more caffeine. Theanine to take the edge off coffee he already drinks. Normal bloods → don’t sell B12 as energy |
| **K** | 55–64 F, pescatarian, HRT, vit D + omega-3, ageing+joints | Creatine for ageing muscle; **no 4000 IU D on top**; ashwagandha/boron penalized; omega-3 catalog gap |

---

## Project layout

```
src/features.rs   shared 18-dim space + cosine
src/persona.rs    quiz records + vectorize()
src/catalog.rs    locked CLD-9 20-active library
src/safety.rs     allergies / meds / conditions / dedup
src/engine.rs     score, dose, assemble 4–8, explain
src/report.rs     CLI + markdown
src/main.rs       cargo run interface
src/lib.rs        crate root
data/catalog.md   human-readable catalog lock
outputs/          committed sample runs
```

Zero runtime dependencies — `cargo run` is the whole product.

---

## Evidence posture

Strength labels are honest: **strong** (caffeine alertness, creatine performance, vitamin D/B12/zinc when deficient), **moderate** (theanine, ashwagandha stress, magnesium if low, citrulline for training, taurine endurance), **limited / mixed / insufficient** (cordyceps, alpha-GPC, ginkgo in healthy adults, boron, DLPA, Himalayan-salt branding).

Sources cited on each line: NIH Office of Dietary Supplements fact sheets, Cochrane (ginkgo; vitamin C and colds), ISSN position stands (caffeine, creatine), and Examine-style trial summaries. We would rather under-claim than write brochure copy.

---

## Live walkthrough (5 minutes)

1. Open `src/persona.rs` `vectorize()` — show Persona A’s sleep-maintenance vs Persona K’s ageing/joint weights.
2. Open `src/catalog.rs` — same 20 rows as the public builder, including the malate quirk and 4000 IU D.
3. Open `src/safety.rs` — allergy hard-block, OSA banner, HRT penalties, multi/D dedup.
4. `cargo run -- --persona B` — clinician flag is the first thing you read.
5. `cargo run -- --demo-allergy` — stack actually changes.

That is the product: **quiz → match from their real library → one safe packet.**
