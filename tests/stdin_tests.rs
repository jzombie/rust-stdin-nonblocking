use std::io::Write;
use std::process::{Command, Stdio};

/// Runs an actual workspace binary (`test_binary` or `tokio-example-app`) and
/// provides input via stdin to observe how the binaries respond, rather than
/// mocking the inputs.
fn run_binary(binary: &str, input: &str) -> String {
    // Ensure all binaries are built first
    let build_status = Command::new("cargo")
        .args(["build", "--workspace"])
        .status()
        .expect("Failed to build workspace binaries");

    if !build_status.success() {
        panic!("Failed to build workspace binaries");
    }

    // Create a mutable Command object
    let mut child = Command::new("cargo");

    // Set the command options based on the binary type
    if binary == "test_binary" {
        child.arg("run").arg("--bin").arg(binary);
    } else if binary == "tokio-example-app" {
        child.arg("run").arg("--package").arg(binary);
    } else {
        panic!("Unknown binary: {}", binary);
    }

    // Set up stdin and stdout for the process
    child.stdin(Stdio::piped()).stdout(Stdio::piped());

    // Spawn the process
    let mut child = child
        .spawn()
        .unwrap_or_else(|_| panic!("Failed to spawn process: {}", binary));

    // Pass the input to the binary if needed
    if let Some(mut stdin) = child.stdin.take() {
        if !input.is_empty() {
            writeln!(stdin, "{}", input).expect("Failed to write to stdin");
        }
        drop(stdin); // Close stdin explicitly to send EOF
    }

    // Capture and return the output
    let output = child.wait_with_output().expect("Failed to read stdout");
    String::from_utf8(output.stdout).expect("Invalid UTF-8 output")
}

/// Test `test_binary` with piped input
#[test]
fn test_piped_input_sync_app() {
    let output = run_binary("test_binary", "test input");
    assert!(output.contains("Received input: Some(\"test input\")"));
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
    assert!(output.contains("Received input: Some(\"test input\")"));
}

/// Test `tokio-example-app` with empty input (should use fallback)
#[test]
fn test_empty_input_tokio_app() {
    let output = run_binary("tokio-example-app", "");
    assert!(
        output.contains("fallback_value"),
        "Expected fallback value but got: {}",
        output
    );
}

/// Test reading the README.md file and ensure all content is captured
#[test]
fn test_multi_line_content() {
    let input = "line1\nline2\r\nline3\rline4\nline5";

    // Run the binary or command and capture the output
    let output = run_binary("test_binary", input);

    assert_eq!(
        output,
        "Received input: Some(\"line1\\nline2\\nline3\\rline4\\nline5\")\n"
    );
}
