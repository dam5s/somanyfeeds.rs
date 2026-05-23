# Somanyfeeds.rs Development Guide

## Build/Configuration Instructions
- **Project Structure**: This is a Rust workspace with two main parts:
    - `apps/somanyfeeds_server`: The main web application using Axum and Askama templates.
    - `pkgs/feeds_processing`: A library package for downloading and parsing RSS/Atom feeds.
- **Environment Variables**:
    - `PORT`: Server port (default: `3000`).
    - `WORKER_INTERVAL_SECONDS`: Background worker refresh interval (default: `300`).
- **Build**: Use `cargo build` from the root to build the entire workspace.
- **Run**: Use `cargo run -p somanyfeeds_server` to start the web server.

### Linux Specific instructions

On linux you may need to install additional dependencies such as `libssl-dev` for building.

## Testing Information
- **Run All Tests**: `cargo test`
- **Run Specific Package Tests**: `cargo test -p somanyfeeds_server` or `cargo test -p feeds_processing`
- **Adding Tests**:
    - Integration tests are located in the `tests/` directory of each package, favor these types of tests.
    - Unit tests using the `#[cfg(test)]` module pattern should be avoided.
- **Test Demonstration**:
  To add and run a new integration test:
    1. Create a new file in `apps/somanyfeeds_server/tests/my_test.rs`.
    2. Add the following content:
       ```rust
       #[test]
       fn test_example() {
           assert_eq!(1 + 1, 2);
       }
       ```
    3. Run it using: `cargo test --test my_test`

## Additional Development Information
- **Architecture**:
    - The server uses a background `Worker` to periodically fetch feeds and update an `ArticlesRepository`.
    - The `router` serves the articles using HTML templates (`askama`).
    - `ArticlesRepository` and `FeedsRepository` are shared via `Arc` for thread-safe access.
- **Code Style**: Follow standard Rust idioms. Use `cargo fmt` for formatting. Agents MUST always run `cargo fmt` after making any changes to the code.
- **Debugging**: The project uses `tracing` for logging. You can control log levels via `RUST_LOG` environment variable.
- **Worker Limits**: Note that the `Worker` currently limits the number of articles per feed to the 20 most recent ones (hardcoded in `worker.rs`).
