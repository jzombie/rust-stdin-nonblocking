use std::io::{self, Write};
use std::thread;
use stdin_nonblocking::spawn_stdin_stream;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

/// Maximum buffer size for async channel
const BUFFER_SIZE: usize = 10;
const FALLBACK_VALUE: &[u8] = b"fallback_value";

#[tokio::main]
async fn main() {
    // Step 1: Start the blocking stdin reader
    let blocking_stdin_stream = spawn_stdin_stream(); // std::sync::mpsc::Receiver<Vec<u8>>

    // Step 2: Create an async Tokio channel
    let (tx, mut rx) = mpsc::channel::<Vec<u8>>(BUFFER_SIZE);

    // Step 3: Spawn a thread to forward data from std::sync::mpsc to Tokio mpsc
    thread::spawn(move || {
        while let Ok(chunk) = blocking_stdin_stream.recv() {
            if tx.blocking_send(chunk).is_err() {
                break; // If the receiver is closed, stop forwarding
            }
        }
    });

    // Step 4: Process the async stream of binary input
    let mut received_any = false;

    while let Some(chunk) = rx.recv().await {
        received_any = true;

        // Simulate async work per chunk
        sleep(Duration::from_millis(100)).await;

        // Print raw binary data as it arrives
        io::stdout()
            .write_all(&chunk)
            .expect("Failed to write output");
    }

    // Step 5: If no input was received, print the fallback value
    if !received_any {
        io::stdout()
            .write_all(FALLBACK_VALUE)
            .expect("Failed to write fallback value");
    }
}
