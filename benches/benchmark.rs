use blake2::{Blake2s256, Digest};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use file_hashing::*;
use std::path::PathBuf;

fn criterion_benchmark(c: &mut Criterion) {
    let mut hash = Blake2s256::new();
    c.bench_function("file_hashing::get_hash_file", |b| {
        b.iter(|| {
            file_hashing::get_hash_file(&PathBuf::from("/home/gladi/test-hashing.txt"), &mut hash)
        })
    });

    c.bench_function("file_hashing::get_hash_files", |b| {
        b.iter(|| {
            let walkdir = walkdir::WalkDir::new("/home/gladi/Pictures");
            let mut paths: Vec<PathBuf> = Vec::new();

            for file in walkdir.into_iter().filter_map(|file| file.ok()) {
                if file.metadata().unwrap().is_file() {
                    paths.push(file.into_path());
                }
            }

            file_hashing::get_hash_files(&paths, &mut hash, 12, |_| {});
        })
    });

    //file_hashing::get_hash_folder(dir, hash, num_threads, progress)
    c.bench_function("file_hashing::get_hash_folder", |b| {
        b.iter(|| {
            file_hashing::get_hash_folder(
                &PathBuf::from("/home/gladi/Pictures"),
                &mut hash,
                12,
                |_| {},
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
