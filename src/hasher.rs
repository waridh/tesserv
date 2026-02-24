/** Hasher library built on the blake3 algorithm. */
use std::path::Path;

pub fn string_of_hash(hash: blake3::Hash) -> String {
    hash.to_hex().to_string()
}

/** Function that hashes the file. The current implementation uses the blake3
algorithm to hash the target file.
 */
pub fn hash_file(path: impl AsRef<Path>) -> Result<String, ()> {
    let mut hasher = blake3::Hasher::new();
    let result = if let Ok(x) = hasher.update_mmap_rayon(path) {
        let done = x.finalize();
        Ok(string_of_hash(done))
    } else {
        Err(())
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_hashing() {
        let filepath_to_b3sum = vec![(
            Path::new("tests/resources/group0/testfile.sch"),
            "62356751b50db378e89a04649d54b6acdad0c9e98eba19827923b86b12ef430b",
        )];
        for (p, h) in filepath_to_b3sum {
            if !p.exists() {
                assert!(false, "the path {} does not exist", p.display());
            }
            let res = hash_file(p);
            if let Ok(x) = res {
                assert_eq!(h, x, "expecting {} and {} to be the same hash", h, x);
            } else {
                assert!(false, "failed to hash {}.", p.display())
            }
        }
    }
}
