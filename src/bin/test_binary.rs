use std::io::{self, Write};
use stdin_nonblocking::get_stdin_or_default;

// Used for integration testing
fn main() {
    let input = get_stdin_or_default(Some(b"fallback_value"));

    // Print raw binary data instead of Debug format
    io::stdout()
        .write_all(&input)
        .expect("Failed to write output");
}
