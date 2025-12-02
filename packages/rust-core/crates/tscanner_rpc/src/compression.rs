use base64::Engine;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{self, Write};

use crate::protocol::Response;

pub fn send_compressed_response(stdout: &mut io::Stdout, response: Response) {
    let Ok(json) = serde_json::to_string(&response) else {
        return;
    };

    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());

    if encoder.write_all(json.as_bytes()).is_err() {
        return;
    }

    let Ok(compressed) = encoder.finish() else {
        return;
    };

    let _ = stdout.write_all(b"GZIP:");
    let encoded = base64::engine::general_purpose::STANDARD.encode(&compressed);
    let _ = stdout.write_all(encoded.as_bytes());
    let _ = stdout.write_all(b"\n");
    let _ = stdout.flush();
}
