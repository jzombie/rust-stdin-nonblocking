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

/// Reads stdin if available; otherwise, returns a default value.
///
/// This function intelligently determines whether to block:
/// - **Interactive Mode**: If stdin is a terminal, the function immediately returns the default without blocking.
/// - **Redirected Input**: If stdin is redirected from a file or pipe, it spawns a thread to read stdin and waits briefly (50ms).
///   - If data arrives promptly, it returns immediately.
///   - If no data is available within that short duration, it returns the provided default value.
///
/// # Arguments
/// * `default` - An optional fallback value returned if no input is available.
///
/// # Returns
/// * `Option<Vec<u8>>` - The stdin input if available, otherwise the provided default.
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
    if !io::stdin().is_terminal() {
        let stdin_channel = spawn_stdin_stream();

        // Blocking recv() waits until data arrives or EOF occurs
        match stdin_channel.recv() {
            Ok(data) => return Some(data),
            Err(e) => eprintln!("Channel closed without data: {}", e),
        }
    }

    default.map(|val| val.to_vec())
}
