use crate::session::Session;
use lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams, Diagnostic, Position, Range,
    TextEdit, Url, WorkspaceEdit,
};
use std::collections::HashMap;
use tscanner_scanner::{ignore_comment, ignore_next_line_comment};

pub fn handle_code_action(params: CodeActionParams, session: &Session) -> Vec<CodeActionOrCommand> {
    if !session.is_initialized() {
        return Vec::new();
    }

    let uri = &params.text_document.uri;
    let mut actions: Vec<CodeActionOrCommand> = Vec::new();

    let Some(diags_with_rules) = session.diagnostics.get(uri) else {
        return actions;
    };

    let Some(doc) = session.get_document(uri) else {
        return actions;
    };
    let content = &doc.content;

    for diagnostic in &params.context.diagnostics {
        let matching = diags_with_rules
            .iter()
            .find(|(d, _)| d.range == diagnostic.range && d.message == diagnostic.message);

        if let Some((_, rule_id)) = matching {
            let line = diagnostic.range.start.line as usize;
            let indentation = get_line_indentation(content, line);

            let ignore_line_action = create_ignore_line_action(
                uri.clone(),
                rule_id,
                line,
                &indentation,
                diagnostic.clone(),
            );
            actions.push(CodeActionOrCommand::CodeAction(ignore_line_action));

            let ignore_rule_action =
                create_ignore_rule_action(uri.clone(), rule_id, diagnostic.clone());
            actions.push(CodeActionOrCommand::CodeAction(ignore_rule_action));

            let ignore_file_action = create_ignore_file_action(uri.clone(), diagnostic.clone());
            actions.push(CodeActionOrCommand::CodeAction(ignore_file_action));
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

fn create_ignore_line_action(
    uri: Url,
    rule_id: &str,
    line: usize,
    indentation: &str,
    diagnostic: Diagnostic,
) -> CodeAction {
    let comment = format!(
        "{}// {} {}\n",
        indentation,
        ignore_next_line_comment(),
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
        title: format!("Ignore {} for this line", rule_id),
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

fn create_ignore_rule_action(uri: Url, rule_id: &str, diagnostic: Diagnostic) -> CodeAction {
    let comment = format!("// {} {}\n", ignore_comment(), rule_id);

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
        title: format!("Ignore {} for entire file", rule_id),
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

fn create_ignore_file_action(uri: Url, diagnostic: Diagnostic) -> CodeAction {
    let comment = format!("// {}\n", ignore_comment());

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
        title: "Ignore tscanner for entire file".to_string(),
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
