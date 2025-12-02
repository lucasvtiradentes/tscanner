mod compression;
mod handlers;
mod protocol;
mod server;
mod state;

pub use protocol::{Notification, Request, Response};
pub use server::RpcServer;

pub fn run_rpc_server() {
    let mut server = RpcServer::new();
    server.run();
}
