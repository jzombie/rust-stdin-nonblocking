use stdin_nonblocking::get_stdin_or_default;

fn main() {
    let input = get_stdin_or_default(Some(b"backup_value"));
    println!("Final input: {:?}", input);
}
