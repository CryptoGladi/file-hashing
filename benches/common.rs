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
