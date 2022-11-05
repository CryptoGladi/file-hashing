# file-hashing

This crate will help you easily get hash from **files** or **folders**

# Example

```rust
let path = PathBuf::from("/home/gladi/test-hashing.txt");

let mut hash = Blake2s256::new();
let result = get_hash_file(&path, &mut hash).unwrap();

assert_eq!(result.len(), 64); // Blake2s256 len == 64
```

> P.S. If the examples from the documentation **do not work**, then you need to look at the **unit tests**

# Motivation

Each time we write our own function to get the hash from a file or folder.
We repeat ourselves and violate the [DRY](https://en.wikipedia.org/wiki/Don't_repeat_yourself) principle.
With this crate you can save a couple of hours

License: **MIT**