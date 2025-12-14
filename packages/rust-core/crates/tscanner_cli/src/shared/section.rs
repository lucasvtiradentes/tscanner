use colored::*;
use tscanner_constants::{icon_error, icon_warning};

pub fn print_section_header(title: &str) {
    println!("{}", title.cyan().bold());
    println!();
}

pub fn print_section_title(title: &str) {
    println!("{}", title.cyan().bold());
}

pub fn print_setting(label: &str, value: &str) {
    println!("  {} {}", format!("{}:", label).dimmed(), value);
}

pub fn print_setting_value<T: std::fmt::Display>(label: &str, value: T) {
    println!("  {} {}", format!("{}:", label).dimmed(), value);
}

pub fn render_notes(notes: &[String]) {
    if !notes.is_empty() {
        println!();
        print_section_header("Notes:");
        for note in notes {
            println!("  {} {}", "ℹ".blue(), note.dimmed());
        }
    }
}

pub fn render_warnings(warnings: &[String]) {
    if !warnings.is_empty() {
        println!();
        print_section_header("Warnings:");
        for warning in warnings {
            println!("  {} {}", icon_warning().yellow(), warning.yellow());
        }
    }
}

pub fn render_runtime_errors(errors: &[String]) {
    if !errors.is_empty() {
        println!();
        print_section_header("Errors:");
        for error in errors {
            println!("  {} {}", icon_error().red(), error.red());
        }
    }
}

pub fn render_messages(notes: &[String], warnings: &[String], errors: &[String]) {
    render_notes(notes);
    render_warnings(warnings);
    render_runtime_errors(errors);
}

pub fn fatal_error_and_exit(message: &str, help_lines: &[&str]) -> ! {
    println!();
    print_section_header("Fatal Error:");
    println!("  {} {}", icon_error().red(), message.red());

    if !help_lines.is_empty() {
        println!();
        for line in help_lines {
            println!("  {}", line);
        }
    }

    println!();
    std::process::exit(1);
}
