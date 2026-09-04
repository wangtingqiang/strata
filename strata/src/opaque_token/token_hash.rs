use secrecy::{ExposeSecret, SecretString};
use sha2::{Digest, Sha256};

use crate::opaque_token::OpaqueToken;

/// 令牌哈希错误。
#[derive(Debug, thiserror::Error)]
pub enum TokenHashError {
    /// 哈希为空。
    #[error("token hash must not be empty")]
    Empty,

    /// 哈希格式非法。
    #[error("invalid token hash format: {message}")]
    InvalidFormat {
        /// 格式错误说明。
        message: String,
    },
}

/// 令牌的 SHA-256 哈希。
#[derive(Debug, Clone)]
pub struct TokenHash(SecretString);

impl PartialEq for TokenHash {
    fn eq(&self, other: &Self) -> bool {
        self.expose_secret() == other.expose_secret()
    }
}

impl Eq for TokenHash {}

impl From<&OpaqueToken> for TokenHash {
    fn from(token: &OpaqueToken) -> Self {
        let digest = hex::encode(Sha256::digest(token.expose_secret()));

        Self(digest.into())
    }
}

impl TryFrom<String> for TokenHash {
    type Error = TokenHashError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(TokenHashError::Empty);
        }
        if trimmed.len() != 64 || !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(TokenHashError::InvalidFormat {
                message: "token hash must be 64 hex characters".to_owned(),
            });
        }

        if trimmed.as_bytes().iter().all(u8::is_ascii_lowercase) {
            Ok(Self(SecretString::from(trimmed)))
        } else {
            Ok(Self(SecretString::from(trimmed.to_ascii_lowercase())))
        }
    }
}

impl TryFrom<&str> for TokenHash {
    type Error = TokenHashError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl TokenHash {
    /// 暴露哈希原文。
    pub fn expose_secret(&self) -> &str {
        self.0.expose_secret()
    }
}

impl From<TokenHash> for SecretString {
    fn from(value: TokenHash) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::opaque_token::OpaqueToken;

    use super::*;

    #[test]
    fn accepts_lowercase_64_hex() {
        let hash = "ab".repeat(32);
        let token_hash = TokenHash::try_from(hash.as_str()).unwrap();
        assert_eq!(token_hash.expose_secret(), hash);
    }

    #[test]
    fn normalizes_uppercase_to_lowercase() {
        let token_hash = TokenHash::try_from("AB".repeat(32)).unwrap();
        assert_eq!(token_hash.expose_secret(), "ab".repeat(32));
    }

    #[test]
    fn rejects_wrong_length() {
        assert!(matches!(
            TokenHash::try_from("abcdef"),
            Err(TokenHashError::InvalidFormat { .. })
        ));
    }

    #[test]
    fn rejects_non_hex() {
        assert!(matches!(
            TokenHash::try_from("zz".repeat(32)),
            Err(TokenHashError::InvalidFormat { .. })
        ));
    }

    #[test]
    fn rejects_empty() {
        assert!(matches!(
            TokenHash::try_from("   "),
            Err(TokenHashError::Empty)
        ));
    }

    #[test]
    fn hashes_opaque_token_to_64_hex() {
        let hash: TokenHash = (&OpaqueToken::generate()).into();
        assert_eq!(hash.expose_secret().len(), 64);
        assert!(hash.expose_secret().chars().all(|c| c.is_ascii_hexdigit()));
    }
}
