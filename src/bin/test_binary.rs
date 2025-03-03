use stdin_nonblocking::get_stdin_or_default;

fn main() {
    let input = get_stdin_or_default("fallback_value");
    println!("Received input: {}", input);
}
