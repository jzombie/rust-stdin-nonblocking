use std::io::{self, Write};
use stdin_nonblocking::get_stdin_or_default;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};

fn main() {
    // Step 1: Read stdin input (binary-safe)
    let input = get_stdin_or_default(Some(b"fallback_value"));

    // Step 2: Start the Tokio runtime and pass the input to async code
    let rt = Runtime::new().unwrap();
    rt.block_on(async_main(input));
}

// Step 3: Define an async function to process binary input
async fn async_main(input: Vec<u8>) {
    // Simulate async work
    sleep(Duration::from_secs(1)).await;

    // Print raw binary data instead of Debug format
    io::stdout()
        .write_all(&input)
        .expect("Failed to write output");
}
