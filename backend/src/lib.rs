pub mod config;
pub mod db;
pub mod grpc;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;
pub mod websockets;

// Re-exporting the main function to be used by `main.rs`
pub use crate::main;
