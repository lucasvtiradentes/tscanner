fn main() {
    tscanner_service::init_logger("rust_server     ");

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--lsp" {
        tscanner_service::log_info("Starting in LSP mode");
        if let Err(e) = tscanner_lsp::run_lsp_server() {
            tscanner_service::log_error(&format!("LSP server error: {}", e));
            std::process::exit(1);
        }
        return;
    }

    tscanner_service::log_info("TScanner server started (JSON-RPC mode)");
    tscanner_rpc::run_rpc_server();
}
