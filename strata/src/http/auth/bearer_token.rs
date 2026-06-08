use axum::http::{HeaderMap, header};

pub trait ExtractBearerToken {
    type Error;

    fn extract_bearer_token(&self) -> Result<String, Self::Error>;
}

impl ExtractBearerToken for HeaderMap {
    type Error = ();

    fn extract_bearer_token(&self) -> Result<String, Self::Error> {
        let authorization = self
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(())?;

        let Some((scheme, token)) = authorization.split_once(' ') else {
            Err(())?
        };
        if !scheme.eq_ignore_ascii_case("bearer") {
            Err(())?
        }
        if token.is_empty() {
            Err(())?
        }

        Ok(token.to_owned())
    }
}
