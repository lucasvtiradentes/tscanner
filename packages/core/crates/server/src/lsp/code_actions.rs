use core::{disable_file_comment, disable_next_line_comment};
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, Diagnostic, Position, Range,
    TextEdit, Url, WorkspaceEdit,
};
use std::collections::HashMap;

use super::state::LspState;

pub fn handle_code_action(params: CodeActionParams, state: &LspState) -> Vec<CodeActionOrCommand> {
    if state.scanner.is_none() {
        return Vec::new();
    }

    let uri = &params.text_document.uri;
    let mut actions: Vec<CodeActionOrCommand> = Vec::new();

    let Some(diags_with_rules) = state.diagnostics.get(uri) else {
        return actions;
    };

    let Some(content) = state.open_files.get(uri) else {
        return actions;
    };

    for diagnostic in &params.context.diagnostics {
        let matching = diags_with_rules
            .iter()
            .find(|(d, _)| d.range == diagnostic.range && d.message == diagnostic.message);

        if let Some((_, rule_id)) = matching {
            let line = diagnostic.range.start.line as usize;
            let indentation = get_line_indentation(content, line);

            let disable_line_action = create_disable_line_action(
                uri.clone(),
                rule_id,
                line,
                &indentation,
                diagnostic.clone(),
            );
            actions.push(CodeActionOrCommand::CodeAction(disable_line_action));

            let disable_file_action =
                create_disable_file_action(uri.clone(), rule_id, diagnostic.clone());
            actions.push(CodeActionOrCommand::CodeAction(disable_file_action));
        }
    }

    actions
}

fn get_line_indentation(content: &str, line: usize) -> String {
    content
        .lines()
        .nth(line)
        .map(|l| {
            let trimmed = l.trim_start();
            l[..l.len() - trimmed.len()].to_string()
        })
        .unwrap_or_default()
}

fn create_disable_line_action(
    uri: Url,
    rule_id: &str,
    line: usize,
    indentation: &str,
    diagnostic: Diagnostic,
) -> CodeAction {
    let comment = format!(
        "{}// {} {}\n",
        indentation,
        disable_next_line_comment(),
        rule_id
    );

    let edit = TextEdit {
        range: Range {
            start: Position {
                line: line as u32,
                character: 0,
            },
            end: Position {
                line: line as u32,
                character: 0,
            },
        },
        new_text: comment,
    };

    let mut changes = HashMap::new();
    changes.insert(uri, vec![edit]);

    CodeAction {
        title: format!("Disable {} for this line", rule_id),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: Some(vec![diagnostic]),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        is_preferred: Some(false),
        ..Default::default()
    }
}

fn create_disable_file_action(uri: Url, rule_id: &str, diagnostic: Diagnostic) -> CodeAction {
    let comment = format!("// {} {}\n", disable_file_comment(), rule_id);

    let edit = TextEdit {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        },
        new_text: comment,
    };

    let mut changes = HashMap::new();
    changes.insert(uri, vec![edit]);

    CodeAction {
        title: format!("Disable {} for entire file", rule_id),
        kind: Some(CodeActionKind::QUICKFIX),
        diagnostics: Some(vec![diagnostic]),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }),
        is_preferred: Some(false),
        ..Default::default()
    }
}
