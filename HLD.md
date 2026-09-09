# High-level design

One diagram per layer. Each layer maps to one `src/` file.

## Module map

```mermaid
flowchart LR
    main["main.rs<br/>CLI args"] --> engine["engine.rs<br/>score + assemble"]
    main --> tui["tui.rs<br/>interactive picker"]
    tui --> engine
    engine --> persona["persona.rs<br/>quiz → vector"]
    engine --> catalog["catalog.rs<br/>20 actives"]
    engine --> safety["safety.rs<br/>allergies/meds/conditions"]
    engine --> features["features.rs<br/>18-dim space + cosine"]
    persona --> features
    catalog --> features
    safety --> persona
    safety --> catalog
    main --> report["report.rs<br/>CLI + markdown"]
    report --> engine

    classDef focal fill:#eb6c36,stroke:#eb6c36,color:#fff
    class engine focal
```

`engine.rs` is the only module every other layer routes through. Nothing outside it writes a `final_score`.

---

## Persona layer (`src/persona.rs`)

Turns one quiz record into an 18-number need vector. Goals set the largest weights; sleep, caffeine, training, and life stage overlay smaller adjustments on top.

```mermaid
flowchart TD
    P["Persona struct<br/>age, sex, sleep, caffeine, training,<br/>hormones, meds, allergies, goals"] --> G["Goals loop<br/>largest weights"]
    G --> S["Sleep profile overlay<br/>night waking, wired at bedtime"]
    S --> C["Caffeine overlay<br/>cups + last dose"]
    C --> T["Training overlay<br/>sessions per week"]
    T --> L["Life-stage overlay<br/>age band, hormones, body weight"]
    L --> D["Existing-supplement dampening<br/>multivitamin → wellness × 0.35"]
    D --> Z["Clamp every dimension to [0.0, 1.0]"]
    Z --> V["18-dim FeatureVec"]
```

---

## Catalog layer (`src/catalog.rs`)

Fixed at compile time. 20 CLD-9 actives, each with one hand-set benefit vector, one or two offered doses, and a category/family tag used later for stack-diversity caps.

```mermaid
flowchart TD
    I["Ingredient record<br/>id, name, category, family"] --> BV["Benefit vector<br/>hand-set, 18 dims"]
    I --> DS["Offered doses<br/>1-2 fixed amounts, no invented doses"]
    I --> FL["Flags<br/>stimulant, contains_caffeine, bulky"]
    I --> AK["Allergen keys<br/>matched against persona.allergies"]
```

---

## Features layer (`src/features.rs`)

Shared vector math. Both persona and catalog vectors live in this same 18-dimension space, so cosine similarity between them is meaningful.

```mermaid
flowchart LR
    N["Named dimensions<br/>energy_stim, sleep_quality, stress, …"] --> Z["zero() / clamp_unit()"]
    Z --> COS["cosine(a, b)<br/>dot product / (‖a‖ · ‖b‖)"]
    COS --> NZ["nonzero_dims()<br/>preview dims ≥ 0.20"]
```

---

## Engine layer (`src/engine.rs`)

Scores every catalog ingredient against the persona vector, applies the safety multiplier, then greedily assembles a 4-8 item stack.

```mermaid
flowchart TD
    V["Persona vector"] --> SC["score_one() per ingredient<br/>0.72×cosine + 0.28×goal_bonus"]
    SC --> SF["safety::evaluate()<br/>severity → multiplier"]
    SF --> FS["final_score = raw × multiplier"]
    FS --> CB["apply_combo_bonuses()<br/>+bonus for working pairs"]
    CB --> RANK["sort by final_score"]
    RANK --> AS["assemble_stack()<br/>1 stimulant · ≤2 bulky · ≤2 vitamins"]
    AS --> OUT["Recommendation<br/>stack + excluded + flags"]
```

---

## Safety layer (`src/safety.rs`)

Runs inside `score_one()`, before ranking. A hard block sets the multiplier to `0.0`. No downstream step can raise it back.

```mermaid
flowchart TD
    IN["Persona + Ingredient"] --> AL{"Allergy match?"}
    AL -->|Yes| HB["HardBlock<br/>multiplier 0.0"]
    AL -->|No| MED{"Medication conflict?<br/>blood thinner, MAOI, HRT"}
    MED -->|Yes| SEV["StrongPenalty<br/>multiplier 0.12"]
    MED -->|No| COND{"Condition flag?<br/>possible OSA, pregnancy"}
    COND -->|Yes| HB
    COND -->|No| EXIST{"Already covered?<br/>existing multivitamin/D"}
    EXIST -->|Yes| MOD["ModeratePenalty<br/>multiplier 0.40"]
    EXIST -->|No| OK["NoteOnly<br/>multiplier 1.0"]

    classDef focal fill:#eb6c36,stroke:#eb6c36,color:#fff
    class HB focal
```

---

## Report / TUI layer (`src/report.rs`, `src/tui.rs`)

Two renderers over the same `Recommendation` struct. Neither one recomputes a score. They only format what `engine.rs` already produced.

```mermaid
flowchart LR
    REC["Recommendation"] --> CLI["render_cli()<br/>plain text, cargo run"]
    REC --> MD["render_markdown()<br/>outputs/*.md"]
    REC --> UI["tui.rs<br/>ratatui widgets, --tui"]
```
