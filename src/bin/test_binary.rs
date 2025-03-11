use stdin_nonblocking::get_stdin_or_default;

// Used for integration testing
fn main() {
    let input = get_stdin_or_default(Some(b"fallback_value"));
    println!("Received input: {:?}", input);
}
