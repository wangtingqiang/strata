use sha2::{Digest, Sha256};

pub fn sha256(input: impl AsRef<[u8]>) -> String {
    let mut h = Sha256::new();
    h.update(input.as_ref());
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}
