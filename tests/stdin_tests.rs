use std::io::Write;
use std::process::{Command, Stdio};

/// Runs an actual workspace binary (`test_binary` or `tokio-example-app`)
/// and provides **binary input** via stdin.
/// Returns **raw bytes** instead of assuming UTF-8.
fn run_binary(binary: &str, input: &[u8]) -> Vec<u8> {
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

    // Pass binary input to the process if needed
    if let Some(mut stdin) = child.stdin.take() {
        if !input.is_empty() {
            stdin
                .write_all(input)
                .expect("Failed to write binary to stdin");
        }
        drop(stdin); // Close stdin explicitly to send EOF
    }

    // Capture binary output
    let output = child.wait_with_output().expect("Failed to read stdout");
    output.stdout // Return raw binary output
}

/// **Test binary input handling with raw data**
#[test]
fn test_binary_input_handling() {
    {
        let binary_input: &[u8] = b"\xDE\xAD\xBE\xEF"; // Arbitrary binary data
        let output_bytes = run_binary("test_binary", binary_input);

        assert_eq!(
            output_bytes, binary_input,
            "Expected output to match input, but got: {:?}",
            output_bytes
        );
    }

    {
        let binary_input: &[u8] = b"\xDE\xAD\xBE\xEF"; // Same binary input for tokio-example-app
        let output_bytes = run_binary("tokio-example-app", binary_input);

        assert_eq!(
            output_bytes, binary_input,
            "Expected output to match input, but got: {:?}",
            output_bytes
        );
    }
}

#[test]
fn test_text_input_handling() {
    {
        let text_input = b"Hello, binary world!";
        let output_bytes = run_binary("test_binary", text_input);

        assert_eq!(
            output_bytes, text_input,
            "Expected output to match input, but got: {:?}",
            output_bytes
        );
    }

    {
        let text_input = b"Hello, binary world!";
        let output_bytes = run_binary("tokio-example-app", text_input);

        assert_eq!(
            output_bytes, text_input,
            "Expected output to match input, but got: {:?}",
            output_bytes
        );
    }
}

#[test]
fn test_empty_input() {
    {
        let output_bytes = run_binary("test_binary", b"");

        assert_eq!(
            output_bytes, b"fallback_value",
            "Expected fallback value but got: {:?}",
            output_bytes
        );
    }

    {
        let output_bytes = run_binary("tokio-example-app", b"");

        assert_eq!(
            output_bytes, b"fallback_value",
            "Expected fallback value but got: {:?}",
            output_bytes
        );
    }
}
