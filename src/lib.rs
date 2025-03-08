#[cfg(doctest)]
doc_comment::doctest!("../README.md");

use std::io::{self, BufRead, IsTerminal};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Spawns a background thread that continuously reads from stdin as a stream.
///
/// This function returns an `mpsc Receiver`, allowing non-blocking polling
/// of stdin input just like `spawn_stdin_channel`.
///
/// **Handling Interactive Mode:**
/// - If stdin is a terminal (interactive mode), this function immediately returns an empty receiver.
/// - This prevents blocking behavior when running interactively.
/// - When reading from a file or pipe, the background thread captures input line by line.
///
/// # Returns
/// A `Receiver<String>` that emits lines from stdin.
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
///         Ok(line) => println!("Received: {}", line),
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
pub fn spawn_stdin_stream() -> Receiver<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // If stdin is a terminal, return early (don't block). This check prevents potential blocking
    // if the program is running interactively (i.e. the user is typing in the terminal).
    if io::stdin().is_terminal() {
        return rx;
    }

    thread::spawn(move || {
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();

        loop {
            let mut buffer = String::new();
            match stdin_lock.read_line(&mut buffer) {
                Ok(0) => break, // EOF detected, exit thread
                Ok(_) => {
                    if tx.send(buffer.trim().to_string()).is_err() {
                        break; // Exit if receiver is dropped
                    }
                }
                Err(_) => break, // Read failure
            }
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
/// - This prevents hanging on waiting for user input in interactive sessions.
/// - When used with redirected input (e.g., from a file or pipe), it collects available input.
///
/// # Arguments
/// * `default` - An optional fallback value returned if no input is available.
///
/// # Returns
/// * `Option<String>` - The trimmed `stdin` input as a `String` if available, or the provided `default` as a `String` if no input is received.
///
/// # Example
/// ```
/// use stdin_nonblocking::get_stdin_or_default;
///
/// let input = get_stdin_or_default(Some("fallback_value"));
///
/// assert_eq!(input, Some("fallback_value".to_string()));
/// ```
pub fn get_stdin_or_default(default: Option<&str>) -> Option<String> {
    let stdin_channel = spawn_stdin_stream();
    let mut input = String::new();

    // Give the reader thread a short time to capture any available input
    thread::sleep(Duration::from_millis(50));

    while let Ok(line) = stdin_channel.try_recv() {
        input.push_str(&line); // Collect all lines
        input.push('\n'); // Add a newline between lines
    }

    // If input was collected, return it. Otherwise, return the default value.
    if !input.trim().is_empty() {
        Some(input.trim().to_string())
    } else {
        default.map(|s| s.to_string())
    }
}
