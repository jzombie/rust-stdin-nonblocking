#[cfg(doctest)]
doc_comment::doctest!("../README.md");

use std::io::{self, IsTerminal, Read};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Spawns a background thread that continuously reads from stdin as a binary stream.
///
/// This function returns an `mpsc Receiver`, allowing non-blocking polling
/// of stdin input just like `spawn_stdin_channel`.
///
/// **Handling Interactive Mode:**
/// - If stdin is a terminal (interactive mode), this function immediately returns an empty receiver.
/// - This prevents blocking behavior when running interactively.
/// - When reading from a file or pipe, the background thread captures input **as raw bytes**.
///
/// # Returns
/// A `Receiver<Vec<u8>>` that emits **binary data** from stdin.
///
/// # Example
/// ```
/// use stdin_nonblocking::spawn_stdin_stream;
/// use std::sync::mpsc::TryRecvError;
/// use std::time::Duration;
///
/// let stdin_stream = spawn_stdin_stream();
///
/// loop {
///     match stdin_stream.try_recv() {
///         Ok(bytes) => println!("Received: {:?}", bytes), // Always raw bytes
///         Err(TryRecvError::Empty) => {
///             // No input yet; continue execution
///         }
///         Err(TryRecvError::Disconnected) => {
///             println!("Input stream closed. Exiting...");
///             break;
///         }
///     }
///     std::thread::sleep(Duration::from_millis(500));
/// }
/// ```
pub fn spawn_stdin_stream() -> Receiver<Vec<u8>> {
    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

    // If stdin is a terminal, return early (no blocking).
    if io::stdin().is_terminal() {
        return rx;
    }

    thread::spawn(move || {
        let mut buffer = Vec::new();
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();

        match stdin_lock.read_to_end(&mut buffer) {
            Ok(0) => (), // EOF, no data
            Ok(_) => {
                let _ = tx.send(buffer); // Send full binary data
            }
            Err(_) => (), // Read failure
        }
    });

    rx
}

/// Reads from stdin if available, otherwise returns a default value.
///
/// **Non-blocking:** This function polls `stdin` once and immediately returns.
/// If no input is available within the polling time, it returns the provided default value.
///
/// **Handling Interactive Mode:**
/// - If running interactively (stdin is a terminal), this function returns the default value immediately.
/// - This prevents hanging while waiting for user input in interactive sessions.
/// - When used with redirected input (e.g., from a file or pipe), it collects available **binary** input.
///
/// # Arguments
/// * `default` - An optional fallback value returned if no input is available.
///
/// # Returns
/// * `Option<Vec<u8>>` - The full stdin input (or default value as bytes).
///
/// # Example
/// ```
/// use stdin_nonblocking::get_stdin_or_default;
///
/// let input = get_stdin_or_default(Some(b"fallback_value"));
///
/// assert_eq!(input, Some(b"fallback_value".to_vec()));
/// ```
pub fn get_stdin_or_default(default: Option<&[u8]>) -> Option<Vec<u8>> {
    let stdin_channel = spawn_stdin_stream();

    // Give the reader thread a short time to capture any available input
    thread::sleep(Duration::from_millis(50));

    if let Ok(data) = stdin_channel.try_recv() {
        return Some(data);
    }

    if default.is_none() {
        return None;
    }

    // No input available, return the default value
    Some(default.unwrap_or(b"").to_vec())
}
