use std::{path::Path, fs::File, io::Read};
use digest::Digest;

pub fn get_hash_file<HashType: Digest + Clone>(path: &Path, hash: &mut HashType) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; 4019];

    loop {
        let i = file.read(&mut buf)?;
        hash.update(&buf[0..i]);

        if i == 0 {
            let hex_lower = data_encoding::HEXLOWER.encode(hash.clone().finalize().as_ref());
            return Ok(hex_lower);
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_hash_file() {
        get_hash_file(Path::from("test.file"), );
        assert_eq!()
    }
}
