use base64::Engine;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{self, Write};

use crate::protocol::Response;

pub fn send_compressed_response(stdout: &mut io::Stdout, response: Response) {
    let serialize_start = std::time::Instant::now();
    let Ok(json) = serde_json::to_string(&response) else {
        return;
    };

    let serialize_time = serialize_start.elapsed();
    let original_size = json.len();

    let compress_start = std::time::Instant::now();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());

    if let Err(e) = encoder.write_all(json.as_bytes()) {
        core::log_error(&format!("Failed to compress: {}", e));
        return;
    }

    let Ok(compressed) = encoder.finish() else {
        return;
    };

    let compress_time = compress_start.elapsed();
    let compressed_size = compressed.len();

    if serialize_time.as_millis() > 50 || compress_time.as_millis() > 50 {
        core::log_debug(&format!(
            "Serialization took {}ms ({}KB), compression took {}ms ({}KB → {}KB, {:.1}%)",
            serialize_time.as_millis(),
            original_size / 1024,
            compress_time.as_millis(),
            original_size / 1024,
            compressed_size / 1024,
            (compressed_size as f64 / original_size as f64) * 100.0
        ));
    }

    let write_start = std::time::Instant::now();
    if let Err(e) = stdout.write_all(b"GZIP:") {
        core::log_error(&format!("Failed to write marker: {}", e));
        return;
    }

    let encoded = base64::engine::general_purpose::STANDARD.encode(&compressed);
    if let Err(e) = stdout.write_all(encoded.as_bytes()) {
        core::log_error(&format!("Failed to write compressed data: {}", e));
        return;
    }

    if let Err(e) = stdout.write_all(b"\n") {
        core::log_error(&format!("Failed to write newline: {}", e));
        return;
    }

    let write_time = write_start.elapsed();

    let flush_start = std::time::Instant::now();
    if let Err(e) = stdout.flush() {
        core::log_error(&format!("Failed to flush stdout: {}", e));
    }
    let flush_time = flush_start.elapsed();

    if write_time.as_millis() > 50 || flush_time.as_millis() > 50 {
        core::log_debug(&format!(
            "Write took {}ms, flush took {}ms",
            write_time.as_millis(),
            flush_time.as_millis()
        ));
    }
}
