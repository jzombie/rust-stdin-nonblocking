# Rust `stdin` Nonblocking

[![made-with-rust][rust-logo]][rust-src-page]
[![crates.io][crates-badge]][crates-page]
[![Documentation][docs-badge]][docs-page]
[![MIT licensed][license-badge]][license-page]


| OS            | Status                                                                               |
|---------------|--------------------------------------------------------------------------------------|
| Ubuntu-latest | [![Ubuntu Tests][ubuntu-latest-badge]][ubuntu-latest-workflow]                       |
| macOS-latest  | [![macOS Tests][macos-latest-badge]][macos-latest-workflow]                          |
| Windows-latest| [![Windows Tests][windows-latest-badge]][windows-latest-workflow]                    |

Dependency-less non-blocking `stdin` reader using background threads. Supports streaming and immediate fallback defaults.

Supports **binary data**, streaming, and immediate fallback defaults.

## Install

```sh
cargo add stdin-nonblocking
```

## Usage

### Get `stdin` or Default

```rust
use stdin_nonblocking::get_stdin_or_default;

// If running in interactive mode (stdin is a terminal),
// `get_stdin_or_default` returns the default value immediately.
let input = get_stdin_or_default(Some(b"fallback_value"));

// Input is always `Vec<u8>`, ensuring binary safety.
assert_eq!(input, b"fallback_value".to_vec());
```

### Read `stdin` as Stream

```rust
use stdin_nonblocking::spawn_stdin_stream;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

// If running in interactive mode (stdin is a terminal),
// `spawn_stdin_stream` returns an empty receiver, meaning no input will be received.
let stdin_stream = spawn_stdin_stream();

loop {
    match stdin_stream.try_recv() {
        Ok(bytes) => println!("Received: {:?}", bytes), // Always raw bytes
        Err(TryRecvError::Empty) => {
            // No input yet; continue execution
        }
        Err(TryRecvError::Disconnected) => {
            println!("Input stream closed. Exiting...");
            break;
        }
    }
    std::thread::sleep(Duration::from_millis(500));
}
```

### Use with Tokio

Refer to the included [Tokio Example App](./tokio-example-app/).

## Related threads
  - https://stackoverflow.com/questions/30012995/how-can-i-read-non-blocking-from-stdin
  - https://www.reddit.com/r/rust/comments/fc71ju/how_to_read_from_stdin_without_blocking/?rdt=55515


## License

[MIT License](LICENSE) (c) 2025 Jeremy Harris.


[rust-src-page]: https://www.rust-lang.org/
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?&logo=Rust

[crates-page]: https://crates.io/crates/stdin-nonblocking
[crates-badge]: https://img.shields.io/crates/v/stdin-nonblocking.svg

[docs-page]: https://docs.rs/stdin-nonblocking
[docs-badge]: https://docs.rs/stdin-nonblocking/badge.svg

[license-page]: ./LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg

[ubuntu-latest-badge]: https://github.com/jzombie/rust-stdin-nonblocking/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20ubuntu-latest)
[ubuntu-latest-workflow]: https://github.com/jzombie/rust-stdin-nonblocking/actions/workflows/rust-tests.yml?query=branch%3Amain

[macos-latest-badge]: https://github.com/jzombie/rust-stdin-nonblocking/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20macos-latest)
[macos-latest-workflow]: https://github.com/jzombie/rust-stdin-nonblocking/actions/workflows/rust-tests.yml?query=branch%3Amain

[windows-latest-badge]: https://github.com/jzombie/rust-stdin-nonblocking/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20windows-latest)
[windows-latest-workflow]: https://github.com/jzombie/rust-stdin-nonblocking/actions/workflows/rust-tests.yml?query=branch%3Amain