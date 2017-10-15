///How to create threads to run multiple pieces of code at the same time
///Message passing concurrency, where channels are used to send messages between threads.
///Shared state concurrency, where multiple threads have access to some piece of data.
///The Sync and Send traits, which allow Rust’s concurrency guarantees to be extended to user-defined types as well as types provided by the standard library.

mod ch16_1_threads;
mod ch16_2_message_passing;
mod ch16_3_shared_state;