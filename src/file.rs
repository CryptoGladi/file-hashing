//! File functions

use super::{DynDigest, IOError, IOErrorKind, ProgressInfo, PAGE_SIZE};
use std::{fs::File, io::Read, path::Path};

/// Get hash from **file**
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use blake2::{Blake2s256, Digest};
/// use file_hashing::get_hash_file;
///
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
    HashType: DynDigest + Clone,
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut buf = [0u8; PAGE_SIZE];

    loop {
        let i = file.read(&mut buf)?;
        hash.update(&buf[0..i]);

        if i == 0 {
            return Ok(crate::encoding::get_lowerhex(hash));
        }
    }
}

/// Get hash from **files**
///
/// # Warning
///
/// if you want to get the hash from a folder then it's better to use this [function](get_hash_folder)
///
/// If you can get all files from a folder with this [function](fs::get_all_file_from_folder)
///
/// # Example
///
/// ```no_run
/// use std::path::PathBuf;
/// use blake2::{Blake2s256, Digest};
/// use file_hashing::{get_hash_files, ProgressInfo};
/// use walkdir::WalkDir;
///
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
///
/// # Error
///
/// * if the **path** variable is empty, the error **IOErrorKind::InvalidInput** will be returned
pub fn get_hash_files<HashType, P>(
    paths: &Vec<P>,
    hash: &mut HashType,
    num_threads: usize,
    progress: impl Fn(ProgressInfo),
) -> Result<String, IOError>
where
    HashType: DynDigest + Clone + std::marker::Send,
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
            hash.update(file_hash.as_bytes());
            Ok(())
        }));
    }

    let mut done_files = 0;
    for job in jobs.into_iter() {
        done_files += 1;

        match job {
            Err(error) => progress(ProgressInfo::Error(error)),
            Ok(_) => progress(ProgressInfo::Yield(done_files)),
        }
    }

    Ok(crate::encoding::get_lowerhex(hash))
}

#[cfg(test)]
mod tests {
    use super::ProgressInfo;
    use crate::fs::extra;
    use blake2::{Blake2s256, Digest};
    use std::path::PathBuf;
    use walkdir::WalkDir;

    #[test]
    fn get_hash_file() {
        let (_temp_dir, path) = extra::generate_random_file(32);

        let mut hash = Blake2s256::new();
        let result =
            super::get_hash_file(&path.to_path_buf(), &mut hash).unwrap();

        println!("result: {}", result);
        assert_eq!(result.len(), 64); // Blake2s256 len == 64
    }

    #[test]
    fn get_hash_files() {
        let (temp_dir, _path) =
            extra::generate_random_folder_with_files(325, 32);

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
}
