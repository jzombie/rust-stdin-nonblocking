use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Runs a workspace binary (`test_binary` or `tokio-example-app`) and passes input via stdin
fn run_binary(binary: &str, input: &str) -> String {
    // Ensure all binaries are built first
    let build_status = Command::new("cargo")
        .args(["build", "--bins"])
        .status()
        .expect("Failed to build workspace binaries");

    if !build_status.success() {
        panic!("Failed to build workspace binaries");
    }

    // Attempt to locate binary using Cargo's runtime variable
    let binary_path = env::var(format!("CARGO_BIN_EXE_{}", binary.replace("-", "_")))
        .ok()
        .or_else(|| {
            // Fallback: Manually construct path inside `target/debug/`
            let mut path = PathBuf::from("target/debug");
            path.push(binary);
            if path.exists() {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("Failed to find binary: {}", binary));

    // Run the binary
    let mut child = Command::new(binary_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to spawn process: {}", binary));

    if let Some(mut stdin) = child.stdin.take() {
        if !input.is_empty() {
            writeln!(stdin, "{}", input).expect("Failed to write to stdin");
        }
        drop(stdin); // Close stdin explicitly to send EOF
    }

    let output = child.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stdout).expect("Invalid UTF-8 output")
}

/// Test `test_binary` with piped input
#[test]
fn test_piped_input_sync_app() {
    let output = run_binary("test_binary", "test input");
    assert!(output.contains("Received input: test input"));
}

/// Test `test_binary` with empty input (should use fallback)
#[test]
fn test_empty_input_sync_app() {
    let output = run_binary("test_binary", "");
    assert!(
        output.contains("fallback_value"),
        "Expected fallback value but got: {}",
        output
    );
}

/// Test `tokio-example-app` with piped input
#[test]
fn test_piped_input_tokio_app() {
    let output = run_binary("tokio-example-app", "test input");
    assert!(output.contains("Received input: test input"));
}

/// Yest `tokio-example-app` with empty input (should use fallback)
#[test]
fn test_empty_input_tokio_app() {
    let output = run_binary("tokio-example-app", "");
    assert!(
        output.contains("fallback_value"),
        "Expected fallback value but got: {}",
        output
    );
}
