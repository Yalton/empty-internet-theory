pub mod error;

// You can add additional utility functions and modules here.
// For example, you might want to add a utility function for generating unique IDs for users and posts,
// or for hashing passwords before storing them in the database.

use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

/// Generates a random alphanumeric string of the given length.
pub fn generate_random_id(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Hashes a password using SHA-256.
pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}

// You can add more utility functions as needed.
