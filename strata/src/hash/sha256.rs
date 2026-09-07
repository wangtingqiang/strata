use sha2::{Digest, Sha256};

/// 计算 SHA-256 摘要（十六进制小写）。
pub fn sha256(input: impl AsRef<[u8]>) -> String {
    hex::encode(Sha256::digest(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_known_digest() {
        assert_eq!(
            sha256("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn output_is_64_lowercase_hex_chars() {
        let digest = sha256(b"hello world");

        assert_eq!(digest.len(), 64);
        assert!(digest.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(!digest.chars().any(|c| c.is_ascii_uppercase()));
    }
}
