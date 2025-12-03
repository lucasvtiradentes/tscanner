fn main() {
    tscanner_service::init_logger("rust_server     ");
    tscanner_service::log_info("TScanner server started");

    if let Err(e) = tscanner_lsp::run_lsp_server() {
        tscanner_service::log_error(&format!("Server error: {}", e));
        std::process::exit(1);
    }
}
