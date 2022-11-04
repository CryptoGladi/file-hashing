use blake2::{Blake2s256, Digest};
use criterion::{criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

mod common;

fn all_benchmark(c: &mut Criterion) {
    let mut hash = Blake2s256::new();
    let (_temp_dir_one_file, path_one_file) =
        common::generate_random_file(32);
    let (temp_dir_many_files, _path_many_files) =
        common::generate_random_folder_with_files(325, 32);

    c.bench_function("file_hashing::get_hash_file", |b| {
        b.iter(|| {
            file_hashing::get_hash_file(
                &path_one_file.to_path_buf(),
                &mut hash,
            )
        })
    });

    c.bench_function("file_hashing::get_hash_files", |b| {
        b.iter(|| {
            let walkdir =
                walkdir::WalkDir::new(temp_dir_many_files.to_path_buf());
            let mut paths: Vec<PathBuf> = Vec::new();

            for file in walkdir.into_iter().filter_map(|file| file.ok()) {
                if file.metadata().unwrap().is_file() {
                    paths.push(file.into_path());
                }
            }

            file_hashing::get_hash_files(&paths, &mut hash, 12, |_| {})
                .unwrap();
        })
    });

    c.bench_function("file_hashing::get_hash_folder", |b| {
        b.iter(|| {
            file_hashing::get_hash_folder(
                &temp_dir_many_files.to_path_buf(),
                &mut hash,
                12,
                |_| {},
            )
            .unwrap();
        })
    });
}

criterion_group!(benches, all_benchmark);
criterion_main!(benches);
