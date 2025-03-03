use stdin_nonblocking::get_stdin_or_default;
use tokio::runtime::Runtime;

fn main() {
    // Step 1: Call `get_stdin_or_default` synchronously to get stdin input
    let input = get_stdin_or_default("fallback_value");

    println!("Received input: {}", input);

    // Step 2: Start the Tokio runtime and pass the input to async code
    let rt = Runtime::new().unwrap();
    rt.block_on(async_main(input));
}

// Step 3: Define an async function to process the input
async fn async_main(input: String) {
    println!("Async processing input: {}", input);

    // Simulating async work
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    println!("Async work complete.");
}
