use colored::*;

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
