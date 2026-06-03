use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use tscanner_constants::{icon_error, icon_progress, icon_skipped, icon_success};
use tscanner_scanner::{AiProgressCallback, AiProgressEvent, AiRuleStatus};

use crate::shared::format_duration;

pub(super) fn build_ai_progress_callback(
    effective_ai_enabled: bool,
    is_json: bool,
    scan_skipped: bool,
) -> Option<AiProgressCallback> {
    if !effective_ai_enabled || is_json || scan_skipped {
        return None;
    }

    let rule_states: Arc<Mutex<HashMap<usize, (String, AiRuleStatus)>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let has_rendered = Arc::new(Mutex::new(false));
    let start_time = Arc::new(Mutex::new(None::<std::time::Instant>));
    let states_clone = rule_states.clone();
    let has_rendered_clone = has_rendered.clone();
    let start_time_clone = start_time.clone();
    Some(Arc::new(move |event: AiProgressEvent| {
        let mut states = states_clone.lock().unwrap();
        let mut rendered = has_rendered_clone.lock().unwrap();
        let mut start = start_time_clone.lock().unwrap();
        if start.is_none() {
            *start = Some(std::time::Instant::now());
        }
        states.insert(
            event.rule_index,
            (event.rule_name.clone(), event.status.clone()),
        );
        if states.len() == event.total_rules {
            let elapsed_ms = start.map(|s| s.elapsed().as_millis()).unwrap_or(0);
            render_ai_progress(&states, event.total_rules, !*rendered, elapsed_ms);
            *rendered = true;
        }
    }))
}

fn render_ai_progress(
    states: &HashMap<usize, (String, AiRuleStatus)>,
    total: usize,
    is_first: bool,
    elapsed_ms: u128,
) {
    let completed = states
        .values()
        .filter(|(_, s)| {
            matches!(
                s,
                AiRuleStatus::Completed { .. } | AiRuleStatus::Failed { .. }
            )
        })
        .count();

    let all_completed = completed == total;

    if all_completed {
        if !is_first {
            eprint!("\x1B[1A");
            eprint!("\x1B[0J");
        }
    } else {
        if !is_first {
            eprint!("\x1B[1A");
            eprint!("\x1B[0J");
        }
        eprintln!(
            "{} {} {}",
            icon_progress(),
            format!("AI rules ({}/{})", completed, total).cyan().bold(),
            format_duration(elapsed_ms).dimmed()
        );
    }

    let _ = io::stderr().flush();
}

pub(super) enum RuleStatus {
    Completed(u128),
    Skipped,
    Error(u128),
}

pub(super) fn render_rules_status(label: &str, count: usize, status: RuleStatus) {
    match status {
        RuleStatus::Completed(duration_ms) => {
            println!(
                "{} {}",
                icon_success().green(),
                format!(
                    "{} ({}) {}",
                    label,
                    count,
                    format_duration(duration_ms).dimmed()
                )
                .cyan()
                .bold()
            );
        }
        RuleStatus::Skipped => {
            println!(
                "{} {}",
                icon_skipped().dimmed(),
                format!("{} ({}) {}", label, count, "skipped".dimmed()).dimmed()
            );
        }
        RuleStatus::Error(duration_ms) => {
            println!(
                "{} {}",
                icon_error().red(),
                format!(
                    "{} ({}) {} {}",
                    label,
                    count,
                    format_duration(duration_ms).dimmed(),
                    "error".red()
                )
                .red()
            );
        }
    }
}
