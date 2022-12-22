//! Folder functions

use super::*;
use std::path::PathBuf;

/// Get hash from **folder**
///
/// This function gets all files from a folder recursively and gets their hash
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use blake2::{Blake2s256, Digest};
/// use file_hashing::get_hash_folder;
///
/// let mut hash = Blake2s256::new();
///
/// let result = get_hash_folder(
///     &PathBuf::from("/home/gladi/Pictures"),
///     &mut hash,
///     12,
///     |_| {},
/// )
/// .unwrap();
///
/// assert_eq!(result.len(), 64); // Blake2s256 len == 64
/// ```
///
/// # Error
///
/// * If the folder **is empty**, the **IOErrorKind::InvalidInput** error will be returned
pub fn get_hash_folder<HashType, P>(
    dir: P,
    hash: &mut HashType,
    num_threads: usize,
    progress: impl Fn(ProgressInfo),
) -> Result<String, IOError>
where
    HashType: DynDigest + Clone + std::marker::Send,
    P: AsRef<Path> + std::marker::Sync,
{
    get_hash_files(
        &fs::get_all_file_from_folder(dir),
        hash,
        num_threads,
        progress,
    )
}

/// Get hash from **folders**
///
/// This function gets all files from a folders recursively and gets their hash
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use blake2::{Blake2s256, Digest};
/// use file_hashing::get_hash_folders;
///
/// let mut hash = Blake2s256::new();
///
/// let result = get_hash_folders(
///     &vec![PathBuf::from("/home/gladi/Pictures"), PathBuf::from("/home/gladi/Hentai")],
///     &mut hash,
///     12,
///     |_| {},
/// )
/// .unwrap();
///
/// assert_eq!(result.len(), 64); // Blake2s256 len == 64
/// ```
///
/// # Error
///
/// * If the folders **is empty**, the **IOErrorKind::InvalidInput** error will be returned
pub fn get_hash_folders<HashType, P>(
    dirs: &Vec<P>,
    hash: &mut HashType,
    num_threads: usize,
    progress: impl Fn(ProgressInfo),
) -> Result<String, IOError>
where
    HashType: DynDigest + Clone + std::marker::Send,
    P: AsRef<Path> + std::marker::Sync,
{
    let mut paths: Vec<PathBuf> = vec![];

    for dir in dirs {
        paths.append(&mut fs::get_all_file_from_folder(dir));
    }

    get_hash_files(&paths, hash, num_threads, progress)
}

#[cfg(test)]
mod tests {
    use crate::fs::extra;
    use blake2::{Blake2s256, Digest};

    #[test]
    fn get_hash_folder() {
        let mut hash = Blake2s256::new();
        let (temp_dir, _path) =
            extra::generate_random_folder_with_files(325, 32);

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

    #[test]
    fn get_hash_folders() {
        let mut hash = Blake2s256::new();
        let (temp_dir1, _path1) =
            extra::generate_random_folder_with_files(325, 32);
        let (temp_dir2, _path2) =
            extra::generate_random_folder_with_files(325, 32);

        let result = super::get_hash_folders(
            &vec![temp_dir1.to_path_buf(), temp_dir2.to_path_buf()],
            &mut hash,
            12,
            |_| {},
        )
        .unwrap();

        println!("result: {}", result);
        assert_eq!(result.len(), 64); // Blake2s256 len == 64
    }
}
