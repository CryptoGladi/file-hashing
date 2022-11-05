//! This crate will help you easily get hash from **files** or **folders**
//!
//! # Example
//!
//! ```no_run
//! use std::path::PathBuf;
//! use blake2::{Blake2s256, Digest};
//! use file_hashing::get_hash_file;
//!
//! let path = PathBuf::from("/home/gladi/test-hashing.txt");
//!
//! let mut hash = Blake2s256::new();
//! let result = get_hash_file(&path, &mut hash).unwrap();
//!
//! assert_eq!(result.len(), 64); // Blake2s256 len == 64
//! ```
//!
//! P.S. If the examples from the documentation **do not work**, then you need to look at the **unit tests**

pub(crate) mod encoding;
pub mod file;
pub mod folder;
pub mod fs;

use digest::Digest;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::path::Path;

pub use file::{get_hash_file, get_hash_files};
pub use folder::{get_hash_folder, get_hash_folders};

const PAGE_SIZE: usize = 4096;

/// Information about progress
///
/// # Example
///
/// ```no_run
/// use file_hashing::ProgressInfo;
///
/// let value_files = 300;
/// let info = ProgressInfo::Yield(1); // Just use a function that accepts **progress**
///
/// match info {
///     ProgressInfo::Yield(done_files) => println!("done files {}/{}", done_files, value_files),
///     ProgressInfo::Error(error) => println!("error: {}", error),
/// }
/// ```
pub enum ProgressInfo {
    /// How many files have we processed
    Yield(u64),

    /// Runtime error log
    Error(IOError),
}
