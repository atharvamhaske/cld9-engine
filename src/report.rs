//! CLI + markdown rendering for a recommendation.

use crate::engine::{Recommendation, ScoredItem};
use crate::safety::Severity;

const DISCLAIMER: &str = "NOT MEDICAL ADVICE. Educational take-home only. Not a diagnosis, prescription, or substitute for a clinician.";

pub fn banner() -> String {
    format!(
        "CLD-9-style recommendation engine  ·  quiz → vectors → safety → 4–8 item daily sachet\n{DISCLAIMER}\nCatalog: 20 actives + offered doses from https://www.cld9.ai/custom  ·  flavors: Orange Popsicle / Unflavored"
    )
}

pub fn render_cli(r: &Recommendation) -> String {
    let mut out = String::new();
    out.push_str(&repeat('=', 78));
    out.push('\n');
    out.push_str(&format!(" {}\n", r.persona_label));
    out.push_str(&format!(
        " Daily sachet · {} actives · goals: {}\n",
        r.stack.len(),
        r.goals
    ));
    out.push_str(&repeat('=', 78));
    out.push('\n');
    out.push_str(&format!("{DISCLAIMER}\n\n"));
    out.push_str(&format!("Quiz snapshot:\n  {}\n\n", r.persona_summary));

    out.push_str("Feature vector (dims ≥ 0.20):\n");
    for (name, val) in &r.vector_preview {
        out.push_str(&format!("  {name:<20} {val:.2}  {}\n", bar(*val)));
    }
    out.push('\n');

    out.push_str("Safety / clinical flags\n");
    for f in &r.flags {
        out.push_str(&format!("  ⚠  {f}\n"));
    }
    out.push('\n');

    out.push_str(&format!(
        "Ranked stack ({} actives, one daily packet)\n",
        r.stack.len()
    ));
    out.push_str(&repeat('-', 78));
    out.push('\n');

    for (i, item) in r.stack.iter().enumerate() {
        out.push_str(&format_item(i + 1, item, false));
        out.push('\n');
    }

    if !r.combo_notes.is_empty() {
        out.push_str("Combinations / packet notes\n");
        for n in &r.combo_notes {
            out.push_str(&format!("  • {n}\n"));
        }
        out.push('\n');
    }

    if !r.catalog_notes.is_empty() {
        out.push_str("Catalog notes\n");
        for n in &r.catalog_notes {
            out.push_str(&format!("  • {n}\n"));
        }
        out.push('\n');
    }

    out.push_str("What safety filtered or down-ranked\n");
    let shown = r.excluded.iter().take(8);
    for item in shown {
        out.push_str(&format!(
            "  {:<16} {:<28}  score {:.2}  cosine {:.2}\n    {}\n",
            item.safety.severity.label(),
            item.ingredient.name,
            item.final_score,
            item.cosine,
            truncate(&item.safety_note, 220)
        ));
    }
    out.push('\n');
    out
}

pub fn render_markdown(r: &Recommendation) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", r.persona_label));
    out.push_str(&format!("**{DISCLAIMER}**\n\n"));
    out.push_str(&format!(
        "Daily sachet stack of **{} actives** (CLD-9 custom library + offered doses only). Goals: **{}**.\n\n",
        r.stack.len(),
        r.goals
    ));
    out.push_str(&format!("> {}\n\n", r.persona_summary));

    out.push_str("## Feature vector\n\n");
    out.push_str("Quiz/persona answers encoded onto the shared 18-dim benefit space (dims ≥ 0.20 shown).\n\n");
    out.push_str("| Dimension | Weight |\n|---|---:|\n");
    for (name, val) in &r.vector_preview {
        out.push_str(&format!("| `{name}` | {val:.2} |\n"));
    }
    out.push('\n');

    out.push_str("## Safety / clinical flags\n\n");
    for f in &r.flags {
        out.push_str(&format!("- {f}\n"));
    }
    out.push('\n');

    out.push_str("## Ranked daily-sachet stack\n\n");
    for (i, item) in r.stack.iter().enumerate() {
        out.push_str(&format_item_md(i + 1, item));
    }

    out.push_str("## Combinations\n\n");
    for n in &r.combo_notes {
        out.push_str(&format!("- {n}\n"));
    }
    out.push('\n');

    out.push_str("## Catalog notes\n\n");
    for n in &r.catalog_notes {
        out.push_str(&format!("- {n}\n"));
    }
    out.push('\n');

    out.push_str("## Safety filter (blocked / penalized)\n\n");
    out.push_str("| Action | Ingredient | Final | Cosine | Why |\n|---|---|---:|---:|---|\n");
    for item in r.excluded.iter().take(10) {
        out.push_str(&format!(
            "| {} | {} | {:.2} | {:.2} | {} |\n",
            item.safety.severity.label(),
            item.ingredient.name,
            item.final_score,
            item.cosine,
            escape_md(&truncate(&item.safety_note, 240))
        ));
    }
    out.push('\n');
    out
}

pub fn render_allergy_demo(
    label: &str,
    allergies: &[&str],
    before: &Recommendation,
    after: &Recommendation,
) -> String {
    let mut out = String::new();
    out.push_str(&repeat('=', 78));
    out.push('\n');
    out.push_str(&format!(" WORKED ALLERGY EXAMPLE  ·  {label}\n"));
    out.push_str(&format!(" allergies += {:?}\n", allergies));
    out.push_str(&repeat('=', 78));
    out.push('\n');
    out.push_str(&format!("Baseline stack: {}\n", stack_line(before)));
    out.push_str(&format!("After allergy:  {}\n\n", stack_line(after)));

    let before_ids: Vec<&str> = before.stack.iter().map(|s| s.ingredient.id).collect();
    let after_ids: Vec<&str> = after.stack.iter().map(|s| s.ingredient.id).collect();
    let removed: Vec<&str> = before_ids
        .iter()
        .filter(|id| !after_ids.contains(id))
        .copied()
        .collect();
    let added: Vec<&str> = after_ids
        .iter()
        .filter(|id| !before_ids.contains(id))
        .copied()
        .collect();

    out.push_str(&format!(
        "Removed because of allergy/safety: {}\n",
        join(&removed)
    ));
    out.push_str(&format!(
        "Filled by next valid catalog pick: {}\n",
        join(&added)
    ));
    if removed.is_empty() {
        out.push_str(
            "Stack composition is unchanged because those actives were already out (other safety rules). The allergy still independently hard-blocks every matching catalog row — including caffeine-family items such as green tea extract — so they cannot re-enter if scoring or lifestyle rules are later relaxed.\n",
        );
    }
    out.push('\n');

    for item in after
        .excluded
        .iter()
        .filter(|e| e.safety.severity == Severity::HardBlock)
    {
        if allergies.iter().any(|a| {
            item.safety_note.to_lowercase().contains(&a.to_lowercase())
                || item
                    .ingredient
                    .allergen_keys
                    .iter()
                    .any(|k| k.contains(&a.to_lowercase()))
                || item
                    .ingredient
                    .id
                    .contains(&a.replace('-', "_").to_lowercase())
        }) || item.safety_note.to_lowercase().contains("allergy")
        {
            out.push_str(&format!(
                "  HARD BLOCK  {} — {}\n",
                item.ingredient.name,
                truncate(&item.safety_note, 260)
            ));
        }
    }
    out.push('\n');
    out.push_str("The rest of the stack is re-ranked from remaining unblocked actives. Goal matching never overrides an allergy hard filter.\n\n");
    out
}

pub fn render_allergy_demo_md(
    label: &str,
    allergies: &[&str],
    before: &Recommendation,
    after: &Recommendation,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Worked allergy example — {label}\n\n"));
    out.push_str(&format!(
        "Same structured quiz as `{label}`, plus `allergies: {allergies:?}`.\n\n"
    ));
    out.push_str(&format!("**Baseline:** {}\n\n", stack_line(before)));
    out.push_str(&format!("**After allergy:** {}\n\n", stack_line(after)));
    out.push_str("Hard-blocked items:\n\n");
    for item in after
        .excluded
        .iter()
        .filter(|e| e.safety.severity == Severity::HardBlock)
        .take(8)
    {
        out.push_str(&format!(
            "- **{}** — {}\n",
            item.ingredient.name, item.safety_note
        ));
    }
    out.push_str("\nAllergy / avoid-list tokens are hard exclusions. Scoring cannot put a matched active back into the sachet.\n");
    out
}

fn format_item(rank: usize, item: &ScoredItem, _md: bool) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "#{rank}  {} — {}   [{}]\n",
        item.ingredient.name,
        item.dose.display(),
        item.ingredient.category.as_str()
    ));
    if let Some(alias) = item.ingredient.alias {
        s.push_str(&format!("    alias: {alias}\n"));
    }
    s.push_str(&format!(
        "    score {:.2}   cosine {:.2}   goal-bonus {:.2}   combo {:+.2}   evidence {}\n",
        item.final_score,
        item.cosine,
        item.goal_bonus,
        item.combo_bonus,
        item.ingredient.evidence.as_str()
    ));
    s.push_str(&format!("    timing: {}\n", item.timing));
    s.push_str(&format!("    why: {}\n", item.why));
    s.push_str(&format!(
        "    evidence: {} — {}\n    sources: {}\n",
        item.ingredient.evidence.as_str(),
        item.ingredient.evidence_note,
        item.ingredient.evidence_sources
    ));
    s.push_str(&format!("    safety: {}\n", item.safety_note));
    s
}

fn format_item_md(rank: usize, item: &ScoredItem) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "### {rank}. {} — {} ({})\n\n",
        item.ingredient.name,
        item.dose.display(),
        item.ingredient.category.as_str()
    ));
    s.push_str(&format!(
        "- **Score** {:.2} · cosine {:.2} · goal bonus {:.2} · combo {:+.2}\n",
        item.final_score, item.cosine, item.goal_bonus, item.combo_bonus
    ));
    s.push_str(&format!("- **Timing:** {}\n", item.timing));
    s.push_str(&format!("- **Why it fits:** {}\n", item.why));
    s.push_str(&format!(
        "- **Evidence ({}):** {} _{}_\n",
        item.ingredient.evidence.as_str(),
        item.ingredient.evidence_note,
        item.ingredient.evidence_sources
    ));
    s.push_str(&format!("- **Safety:** {}\n\n", item.safety_note));
    s
}

fn stack_line(r: &Recommendation) -> String {
    r.stack
        .iter()
        .map(|s| format!("{} {}", s.ingredient.name, s.dose.display()))
        .collect::<Vec<_>>()
        .join("  ·  ")
}

fn join(ids: &[&str]) -> String {
    if ids.is_empty() {
        "(none — size held by remaining picks)".into()
    } else {
        ids.join(", ")
    }
}

fn bar(v: f32) -> String {
    let n = (v * 16.0).round() as usize;
    format!("[{}{}]", repeat('#', n), repeat('·', 16 - n))
}

fn repeat(c: char, n: usize) -> String {
    std::iter::repeat(c).take(n).collect()
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let t: String = s.chars().take(max.saturating_sub(1)).collect();
        format!("{t}…")
    }
}

fn escape_md(s: &str) -> String {
    s.replace('|', "\\|")
}
