//! This crate will help you easily get hash from files or folders
//!
//! # Example
//!
//! ```ignore
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

use digest::Digest;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use std::path::Path;
use std::{fs::File, io::Read, path::PathBuf};

/// Information about progress
pub enum ProgressInfo {
    Yield(u64),
    Error(IOError),
}

/// Get hash from **file**
///
/// # Example
///
/// ```ignore
/// let path = PathBuf::from("/home/gladi/test-hashing.txt");
///
/// let mut hash = Blake2s256::new();
/// let result = get_hash_file(&path, &mut hash).unwrap();
///
/// assert_eq!(result.len(), 64); // Blake2s256 len == 64
/// ```
pub fn get_hash_file<HashType, P>(
    path: P,
    hash: &mut HashType,
) -> Result<String, IOError>
where
    HashType: Digest + Clone,
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut buf = [0u8; 4019];

    loop {
        let i = file.read(&mut buf)?;
        hash.update(&buf[0..i]);

        if i == 0 {
            return Ok(encoding::get_lowerhex(hash));
        }
    }
}

/// Get hash from **files**
///
/// if you want to get the hash from a folder then it's better to use this [function](get_hash_folder)
///
/// # Example
///
/// ```ignore
/// let walkdir = WalkDir::new("/home/gladi/Pictures");
/// let mut paths: Vec<PathBuf> = Vec::new();
///
/// for file in walkdir.into_iter().filter_map(|file| file.ok()) {
///     if file.metadata().unwrap().is_file() {
///         paths.push(file.into_path());
///     }
/// }
///
/// let mut hash = Blake2s256::new();
/// let result = get_hash_files(&paths, &mut hash, 4, |info| match info {
///     ProgressInfo::Yield(done_files) => {
///         println!("done files {}/{}", done_files, paths.len())
///     }
///     ProgressInfo::Error(error) => println!("error: {}", error),
/// })
/// .unwrap();
///
/// println!("result: {}", result);
/// assert_eq!(result.len(), 64); // Blake2s256 len == 64
/// ```
pub fn get_hash_files<HashType, P>(
    paths: &Vec<P>,
    hash: &mut HashType,
    num_threads: usize,
    progress: impl Fn(ProgressInfo),
) -> Result<String, IOError>
where
    HashType: Digest + Clone + std::marker::Send,
    P: AsRef<Path> + std::marker::Sync,
{
    if paths.is_empty() {
        return Err(IOError::from(IOErrorKind::InvalidInput));
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();
    let mut jobs = Vec::with_capacity(paths.len());

    for path in paths.iter() {
        jobs.push(pool.install(|| -> Result<(), std::io::Error> {
            let file_hash = get_hash_file(path, hash)?;
            hash.update(file_hash);
            Ok(())
        }));
    }

    let mut done_files = 0u64;
    for job in jobs.into_iter() {
        done_files += 1;

        match job {
            Err(error) => progress(ProgressInfo::Error(error)),
            Ok(_) => progress(ProgressInfo::Yield(done_files)),
        }
    }

    Ok(encoding::get_lowerhex(hash))
}

/// Get hash from **folder**
///
/// This function gets all files from a folder recursively and gets their hash
///
/// # Example
///
/// ```ignore
/// let mut hash = Blake2s256::new();
///
/// let result = super::get_hash_folder(
///     &PathBuf::from("/home/gladi/Pictures"),
///     &mut hash,
///     12,
///     |_| {},
/// )
/// .unwrap();
///
/// assert_eq!(result.len(), 64); // Blake2s256 len == 64
/// ```
pub fn get_hash_folder<HashType, P>(
    dir: P,
    hash: &mut HashType,
    num_threads: usize,
    progress: impl Fn(ProgressInfo),
) -> Result<String, IOError>
where
    HashType: Digest + Clone + std::marker::Send,
    P: AsRef<Path> + std::marker::Sync,
{
    let walkdir = walkdir::WalkDir::new(dir);
    let mut paths: Vec<PathBuf> = Vec::new();

    for file in walkdir.into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            paths.push(file.into_path());
        }
    }

    get_hash_files(&paths, hash, num_threads, progress)
}

#[cfg(test)]
mod tests {
    pub mod common {
        use assert_fs::{fixture::ChildPath, prelude::*};
        use rand::Rng;
        use std::{cmp, fs::File, io::BufWriter, io::Write};

        pub fn generate_random_file(
            size: usize,
        ) -> (assert_fs::TempDir, ChildPath) {
            let temp = assert_fs::TempDir::new().unwrap();
            let input_file = temp.child("random_file.txt");

            let f = File::create(input_file.path()).unwrap();
            let mut writer = BufWriter::new(f);

            let mut rng = rand::thread_rng();
            let mut buffer = [0; 1024];
            let mut remaining_size = size;

            while remaining_size > 0 {
                let to_write = cmp::min(remaining_size, buffer.len());
                let buffer = &mut buffer[..to_write];
                rng.fill(buffer);
                writer.write(buffer).unwrap();

                remaining_size -= to_write;
            }

            return (temp, input_file);
        }

        pub fn generate_random_folder_with_files(
            value_files: usize,
            size: usize,
        ) -> (assert_fs::TempDir, Vec<ChildPath>) {
            let temp = assert_fs::TempDir::new().unwrap();
            let mut input_files = Vec::with_capacity(value_files);

            for i in 0..value_files {
                let input_file = temp.child(format!("random_file_{}.txt", i));

                let f = File::create(input_file.path()).unwrap();
                let mut writer = BufWriter::new(f);

                let mut rng = rand::thread_rng();
                let mut buffer = [0; 1024];
                let mut remaining_size = size;

                while remaining_size > 0 {
                    let to_write = cmp::min(remaining_size, buffer.len());
                    let buffer = &mut buffer[..to_write];
                    rng.fill(buffer);
                    writer.write(buffer).unwrap();

                    remaining_size -= to_write;
                }

                input_files.push(input_file.into());
            }

            return (temp, input_files);
        }
    }

    use super::ProgressInfo;
    use blake2::{Blake2s256, Digest};
    use std::path::PathBuf;
    use walkdir::WalkDir;

    #[test]
    fn get_hash_file() {
        let (_temp_dir, path) = common::generate_random_file(32);

        let mut hash = Blake2s256::new();
        let result =
            super::get_hash_file(&path.to_path_buf(), &mut hash).unwrap();

        println!("result: {}", result);
        assert_eq!(result.len(), 64); // Blake2s256 len == 64
    }

    #[test]
    fn get_hash_files() {
        let (temp_dir, _path) =
            common::generate_random_folder_with_files(325, 32);

        let walkdir = WalkDir::new(temp_dir.to_path_buf());
        let mut paths: Vec<PathBuf> = Vec::new();

        for file in walkdir.into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                paths.push(file.into_path());
            }
        }

        let mut hash = Blake2s256::new();
        let result =
            super::get_hash_files(&paths, &mut hash, 4, |info| match info {
                ProgressInfo::Yield(done_files) => {
                    println!("done files {}/{}", done_files, paths.len())
                }
                ProgressInfo::Error(error) => println!("error: {}", error),
            })
            .unwrap();

        println!("result: {}", result);
        assert_eq!(result.len(), 64); // Blake2s256 len == 64
    }

    #[test]
    fn get_hash_folder() {
        // TODO
        let mut hash = Blake2s256::new();
        let (temp_dir, _path) =
            common::generate_random_folder_with_files(325, 32);

        let result = super::get_hash_folder(
            &temp_dir.to_path_buf(),
            &mut hash,
            12,
            |_| {},
        )
        .unwrap();

        println!("result: {}", result);
        assert_eq!(result.len(), 64); // Blake2s256 len == 64
    }
}
