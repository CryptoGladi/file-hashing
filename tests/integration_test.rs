pub mod common {
    use assert_fs::{fixture::ChildPath, prelude::*};
    use rand::Rng;
    use std::{cmp, fs::File, io::BufWriter, io::Write};
    
    pub fn generate_random_file(size: usize) -> (assert_fs::TempDir, ChildPath) {
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

use blake2::{Blake2s256, Digest};
use file_hashing::ProgressInfo;
use std::path::PathBuf;
use walkdir::WalkDir;

#[test]
fn get_hash_file() {
    let (_temp_dir, path) = common::generate_random_file(32);

    let mut hash = Blake2s256::new();
    println!("llll: {}", path.display());
    let result =
        file_hashing::get_hash_file(&path.to_path_buf(), &mut hash).unwrap();

        println!("result: {}", result);
    assert_eq!(result.len(), 64); // Blake2s256 len == 64
}

#[test]
fn get_hash_files() {
    let (temp_dir, _path) = common::generate_random_folder_with_files(325, 32);

    let walkdir = WalkDir::new(temp_dir.to_path_buf());
    let mut paths: Vec<PathBuf> = Vec::new();

    for file in walkdir.into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            paths.push(file.into_path());
        }
    }

    let mut hash = Blake2s256::new();
    let result =
        file_hashing::get_hash_files(&paths, &mut hash, 4, |info| match info {
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
fn get_hash_folder() { // TODO
 let mut hash = Blake2s256::new();
 let (temp_dir, _path) = common::generate_random_folder_with_files(325, 32);

 let result = file_hashing::get_hash_folder(
         &temp_dir.to_path_buf(),
     &mut hash,
     12,
     |_| {},
 )
 .unwrap();

 println!("result: {}", result);
 assert_eq!(result.len(), 64); // Blake2s256 len == 64
}
