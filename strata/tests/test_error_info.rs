use strata::error::{ErrorInfo, ErrorKind};
use thiserror::Error;

fn assert_msg(err: &impl ErrorInfo, expected: &str) {
    assert_eq!(err.message(), expected);
}

#[test]
fn test_kind() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("validation error")]
        #[info(kind = "Validation", code = "E001", message = "validation error")]
        Validation,

        #[error("not found: {0}")]
        #[info(kind = "NotFound", code = "E002", message = "resource {0} not found")]
        NotFound(String),

        #[error("access denied for {user}")]
        #[info(
            kind = "AccessDenied",
            code = "E003",
            message = "user {user} is not authorized"
        )]
        AccessDenied { user: String },
    }

    assert_eq!(E::Validation.kind(), ErrorKind::Validation);
    assert_eq!(E::NotFound("x".into()).kind(), ErrorKind::NotFound);
    assert_eq!(
        E::AccessDenied { user: "x".into() }.kind(),
        ErrorKind::AccessDenied,
    );
}

#[test]
fn test_code() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("validation")]
        #[info(kind = "Validation", code = "E001", message = "validation")]
        Validation,

        #[error("internal")]
        #[info(kind = "Internal", code = "E002", message = "internal")]
        Internal,
    }

    assert_eq!(E::Validation.code(), "E001");
    assert_eq!(E::Internal.code(), "E002");
}

#[test]
fn test_message_static() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("rate limit exceeded")]
        #[info(kind = "RateLimited", code = "E001", message = "too many requests")]
        RateLimited,
    }

    assert_msg(&E::RateLimited, "too many requests");
}

#[test]
fn test_message_positional() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("not found: {0}")]
        #[info(kind = "NotFound", code = "E001", message = "resource {0} not found")]
        NotFound(String),

        #[error("conflict: {0} {1}")]
        #[info(kind = "Conflict", code = "E002", message = "conflict: {0} and {1}")]
        Conflict(String, String),
    }

    assert_msg(&E::NotFound("user-1".into()), "resource user-1 not found");
    assert_msg(&E::Conflict("a".into(), "b".into()), "conflict: a and b");
}

#[test]
fn test_message_named() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("invalid: {field}")]
        #[info(kind = "Validation", code = "E001", message = "invalid value: {field}")]
        Invalid { field: String },
    }

    assert_msg(
        &E::Invalid {
            field: "email".into(),
        },
        "invalid value: email",
    );
}

#[test]
fn test_message_named_multiple() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("name: {first} {last}")]
        #[info(
            kind = "Validation",
            code = "E001",
            message = "full name: {first} {last}"
        )]
        Name { first: String, last: String },
    }

    assert_msg(
        &E::Name {
            first: "Alice".into(),
            last: "Bob".into(),
        },
        "full name: Alice Bob",
    );
}

#[test]
fn test_message_repeated_field() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("value: {0}")]
        #[info(kind = "Validation", code = "E001", message = "{0} -> {0}")]
        Dup(String),
    }

    assert_msg(&E::Dup("x".into()), "x -> x");
}

#[test]
fn test_message_escaped_braces() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("key: {key}")]
        #[info(kind = "Validation", code = "E001", message = "{{key}}: {key}")]
        InvalidKey { key: String },
    }

    assert_msg(
        &E::InvalidKey {
            key: "email".into(),
        },
        "{key}: email",
    );
}

#[test]
fn test_message_named_partial_refs() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("limit {name} exceeded")]
        #[info(
            kind = "RateLimited",
            code = "E001",
            message = "{name} rate limit exceeded"
        )]
        RateLimit {
            name: String,
            max: u64,
            window_secs: u64,
        },
    }

    assert_msg(
        &E::RateLimit {
            name: "login".into(),
            max: 5,
            window_secs: 60,
        },
        "login rate limit exceeded",
    );
}

#[test]
fn test_message_unnamed_partial_refs() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("too large: {0}")]
        #[info(
            kind = "Validation",
            code = "E001",
            message = "value {0} exceeds limit"
        )]
        TooLarge(u64, u64, u64),
    }

    assert_msg(&E::TooLarge(100, 50, 30), "value 100 exceeds limit");
}

#[test]
fn test_is_severe() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("internal")]
        #[info(kind = "Internal", code = "E001", message = "internal error")]
        Internal,

        #[error("technical")]
        #[info(kind = "Technical", code = "E002", message = "technical error")]
        Technical,

        #[error("unexpected")]
        #[info(kind = "Unexpected", code = "E003", message = "unexpected error")]
        Unexpected,

        #[error("validation")]
        #[info(kind = "Validation", code = "E004", message = "validation error")]
        Validation,

        #[error("not found")]
        #[info(kind = "NotFound", code = "E005", message = "not found")]
        NotFound,
    }

    assert!(E::Internal.is_severe());
    assert!(E::Technical.is_severe());
    assert!(E::Unexpected.is_severe());
    assert!(!E::Validation.is_severe());
    assert!(!E::NotFound.is_severe());
}

#[test]
fn test_multiple_variants() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E {
        #[error("invalid: {0}")]
        #[info(kind = "Validation", code = "E100", message = "invalid: {0}")]
        Invalid(String),

        #[error("not found: {id}")]
        #[info(kind = "NotFound", code = "E101", message = "resource {id} not found")]
        NotFound { id: u64 },

        #[error("internal error")]
        #[info(kind = "Internal", code = "E102", message = "internal server error")]
        Internal,
    }

    let e = E::Invalid("bad".into());
    assert_eq!(e.kind(), ErrorKind::Validation);
    assert_eq!(e.code(), "E100");
    assert_msg(&e, "invalid: bad");

    let e = E::NotFound { id: 42 };
    assert_eq!(e.kind(), ErrorKind::NotFound);
    assert_eq!(e.code(), "E101");
    assert_msg(&e, "resource 42 not found");

    let e = E::Internal;
    assert_eq!(e.kind(), ErrorKind::Internal);
    assert_eq!(e.code(), "E102");
    assert_msg(&e, "internal server error");
}

#[test]
fn test_transparent_tuple() {
    #[derive(Debug, Error, ErrorInfo)]
    enum Inner {
        #[error("validation failed: {0}")]
        #[info(kind = "Validation", code = "I001", message = "内部验证错误: {0}")]
        Validate(String),
    }

    let inner = Inner::Validate("email".into());
    assert_eq!(inner.kind(), ErrorKind::Validation);
    assert_eq!(inner.code(), "I001");
    assert_msg(&inner, "内部验证错误: email");
}

#[test]
fn test_transparent_delegation() {
    #[derive(Debug, Error, ErrorInfo)]
    enum Inner {
        #[error("invalid value")]
        #[info(kind = "Validation", code = "I001", message = "invalid value")]
        Invalid,
    }

    #[derive(Debug, Error, ErrorInfo)]
    enum Outer {
        #[error("{0}")]
        #[info(transparent)]
        Wrap(Inner),
    }

    let e = Outer::Wrap(Inner::Invalid);
    assert_eq!(e.kind(), ErrorKind::Validation);
    assert_eq!(e.code(), "I001");
    assert_msg(&e, "invalid value");
}

#[test]
fn test_transparent_named_field() {
    #[derive(Debug, Error, ErrorInfo)]
    enum Inner {
        #[error("not found: {0}")]
        #[info(kind = "NotFound", code = "I002", message = "资源 {0} 未找到")]
        NotFound(String),
    }

    #[derive(Debug, Error, ErrorInfo)]
    enum Outer {
        #[error("{source}")]
        #[info(transparent)]
        Wrap { source: Inner },
    }

    let e = Outer::Wrap {
        source: Inner::NotFound("user-1".into()),
    };
    assert_eq!(e.kind(), ErrorKind::NotFound);
    assert_eq!(e.code(), "I002");
    assert_msg(&e, "资源 user-1 未找到");
}

#[test]
fn test_transparent_mixed_variants() {
    #[derive(Debug, Error, ErrorInfo)]
    enum Inner {
        #[error("access denied")]
        #[info(kind = "AccessDenied", code = "I003", message = "拒绝访问")]
        Denied,
    }

    #[derive(Debug, Error, ErrorInfo)]
    enum Outer {
        #[error("unauthenticated")]
        #[info(kind = "Unauthenticated", code = "O001", message = "未认证")]
        Unauthenticated,

        #[error("{0}")]
        #[info(transparent)]
        Wrap(Inner),
    }

    let e = Outer::Unauthenticated;
    assert_eq!(e.kind(), ErrorKind::Unauthenticated);
    assert_eq!(e.code(), "O001");
    assert_msg(&e, "未认证");

    let e = Outer::Wrap(Inner::Denied);
    assert_eq!(e.kind(), ErrorKind::AccessDenied);
    assert_eq!(e.code(), "I003");
    assert_msg(&e, "拒绝访问");
}

#[test]
fn test_transparent_multi_field_inner() {
    #[derive(Debug, Error, ErrorInfo)]
    enum Inner {
        #[error("conflict: {0} / {1}")]
        #[info(kind = "Conflict", code = "I004", message = "冲突: {0} 和 {1}")]
        Conflict(String, String),
    }

    #[derive(Debug, Error, ErrorInfo)]
    enum Outer {
        #[error("{0}")]
        #[info(transparent)]
        Wrap(Inner),
    }

    let e = Outer::Wrap(Inner::Conflict("a".into(), "b".into()));
    assert_eq!(e.kind(), ErrorKind::Conflict);
    assert_eq!(e.code(), "I004");
    assert_msg(&e, "冲突: a 和 b");
}

#[test]
fn test_generic() {
    #[derive(Debug, Error, ErrorInfo)]
    enum E<T: std::fmt::Debug + std::fmt::Display> {
        #[error("{0}")]
        #[info(kind = "Internal", code = "E001", message = "internal: {0}")]
        Internal(T),
    }

    assert_msg(&E::Internal("boom"), "internal: boom");
}
