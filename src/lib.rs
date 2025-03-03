use std::io::{self, BufRead};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

/// Spawns a background thread that continuously reads from stdin as a stream.
///
/// This function returns an `mpsc Receiver`, allowing non-blocking polling
/// of stdin input just like `spawn_stdin_channel`.
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
/// fn main() {
///     let stdin_stream = spawn_stdin_stream();
///
///     loop {
///         match stdin_stream.try_recv() {
///             Ok(line) => println!("Received: {}", line),
///             Err(TryRecvError::Empty) => {
///                 // No input yet; continue execution
///             }
///             Err(TryRecvError::Disconnected) => {
///                 println!("Input stream closed. Exiting...");
///                 break;
///             }
///         }
///         std::thread::sleep(Duration::from_millis(500));
///     }
/// }
/// ```
pub fn spawn_stdin_stream() -> Receiver<String> {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

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
///
/// # Arguments
/// * `default` - A fallback value returned if no input is available.
///
/// # Returns
/// * `String` - Trimmed stdin input or `default` if no input is received.
///
/// # Example
/// ```
/// use stdin_nonblocking::get_stdin_or_default;
///
/// fn main() {
///     let input = get_stdin_or_default("fallback_value");
///     println!("Final input: {}", input);
/// }
/// ```
pub fn get_stdin_or_default(default: &str) -> String {
    let stdin_channel = spawn_stdin_stream();

    // Give the reader thread a short time to capture any available input
    thread::sleep(Duration::from_millis(50));

    match stdin_channel.try_recv() {
        Ok(input) if !input.trim().is_empty() => input,
        _ => default.to_string(),
    }
}
