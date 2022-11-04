pub(crate) mod encoding;

use digest::Digest;
use std::{fs::File, io::Read, path::PathBuf};

pub enum ProgressInfo {
    Yield(u64),
    Error(std::io::Error),
}

pub fn get_hash_file<HashType: Digest + Clone>(
    path: &PathBuf,
    hash: &mut HashType,
) -> Result<String, std::io::Error> {
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

pub fn get_hash_files<HashType: Digest + Clone + std::marker::Send>(
    paths: &Vec<PathBuf>,
    hash: &mut HashType,
    num_threads: usize,
    progress: impl Fn(ProgressInfo),
) -> Option<String> {
    if paths.is_empty() {
        return None;
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

    Some(encoding::get_lowerhex(hash))
}

pub fn get_hash_folder<HashType: Digest + Clone + std::marker::Send>(
    dir: &PathBuf,
    hash: &mut HashType,
    num_threads: usize,
    progress: impl Fn(ProgressInfo),
) -> Option<String> {
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
    use crate::ProgressInfo;
    use blake2::{Blake2s256, Digest};
    use std::path::PathBuf;
    use walkdir::WalkDir;

    #[test]
    fn get_hash_file() {
        let path = PathBuf::from("/home/gladi/test-hashing.txt");

        let mut hash = Blake2s256::new();
        let result = super::get_hash_file(&path, &mut hash).unwrap();

        assert_eq!(result.len(), 64); // Blake2s256 len == 64
    }

    #[test]
    fn get_hash_files() {
        let walkdir = WalkDir::new("/home/gladi/Pictures");
        let mut paths: Vec<PathBuf> = Vec::new();

        for file in walkdir.into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                paths.push(file.into_path());
            }
        }

        let mut hash = Blake2s256::new();
        let result = super::get_hash_files(&paths, &mut hash, 4, |info| match info {
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
