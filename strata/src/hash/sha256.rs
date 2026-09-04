use sha2::{Digest, Sha256};

/// 计算 SHA-256 摘要（十六进制小写）。
pub fn sha256(input: impl AsRef<[u8]>) -> String {
    hex::encode(Sha256::digest(input))
}
