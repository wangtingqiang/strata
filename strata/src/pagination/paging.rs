use strata_error::ErrorInfo;

pub const DEFAULT_PAGE: u64 = 1;
pub const DEFAULT_PAGE_SIZE: u64 = 10;
pub const MAX_PAGE_SIZE: u64 = 100;
pub const DEFAULT_LIMIT: u64 = 10;
pub const MAX_LIMIT: u64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paging {
    page: u64,
    page_size: u64,
    need_total: bool,
}

#[derive(Debug, Clone, Copy, thiserror::Error, PartialEq, Eq, ErrorInfo)]
pub enum PagingError {
    #[error("page must be greater than 0")]
    #[info(kind = "Validation", code = "INVALID_PAGE", message = "页码必须大于 0")]
    InvalidPage,

    #[error("page size must be greater than 0")]
    #[info(
        kind = "Validation",
        code = "INVALID_PAGE_SIZE",
        message = "每页条数必须大于 0"
    )]
    InvalidPageSize,

    #[error("page size must not exceed {MAX_PAGE_SIZE}")]
    #[info(
        kind = "Validation",
        code = "PAGE_SIZE_EXCEEDED",
        message = "每页条数不能超过 {MAX_PAGE_SIZE}"
    )]
    PageSizeExceeded,

    #[error("limit must be greater than 0")]
    #[info(
        kind = "Validation",
        code = "INVALID_LIMIT",
        message = "查询条数必须大于 0"
    )]
    InvalidLimit,

    #[error("limit must not exceed {MAX_LIMIT}")]
    #[info(
        kind = "Validation",
        code = "LIMIT_EXCEEDED",
        message = "查询条数不能超过 {MAX_LIMIT}"
    )]
    LimitExceeded,
}

impl Paging {
    pub fn new(page: u64, page_size: u64, need_total: bool) -> Result<Self, PagingError> {
        if page == 0 {
            return Err(PagingError::InvalidPage);
        }
        if page_size == 0 {
            return Err(PagingError::InvalidPageSize);
        }
        if page_size > MAX_PAGE_SIZE {
            return Err(PagingError::PageSizeExceeded);
        }

        Ok(Self {
            page,
            page_size,
            need_total,
        })
    }

    pub fn from_optional(
        page: Option<u64>,
        page_size: Option<u64>,
        need_total: bool,
    ) -> Result<Self, PagingError> {
        Self::from_optional_with_defaults(
            page,
            page_size,
            need_total,
            DEFAULT_PAGE,
            DEFAULT_PAGE_SIZE,
        )
    }

    pub fn from_optional_with_defaults(
        page: Option<u64>,
        page_size: Option<u64>,
        need_total: bool,
        default_page: u64,
        default_page_size: u64,
    ) -> Result<Self, PagingError> {
        Self::new(
            page.unwrap_or(default_page),
            page_size.unwrap_or(default_page_size),
            need_total,
        )
    }

    pub fn page(&self) -> u64 {
        self.page
    }

    pub fn page_size(&self) -> u64 {
        self.page_size
    }

    pub fn need_total(&self) -> bool {
        self.need_total
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorPaging<C> {
    limit: u64,
    cursor: Option<C>,
}

impl<C> CursorPaging<C> {
    pub fn new(limit: u64, cursor: Option<C>) -> Result<Self, PagingError> {
        if limit == 0 {
            return Err(PagingError::InvalidLimit);
        }
        if limit > MAX_LIMIT {
            return Err(PagingError::LimitExceeded);
        }

        Ok(Self { limit, cursor })
    }

    pub fn from_optional(limit: Option<u64>, cursor: Option<C>) -> Result<Self, PagingError> {
        Self::new(limit.unwrap_or(DEFAULT_LIMIT), cursor)
    }

    pub fn limit(&self) -> u64 {
        self.limit
    }

    pub fn cursor(&self) -> Option<&C> {
        self.cursor.as_ref()
    }

    pub fn into_parts(self) -> (u64, Option<C>) {
        (self.limit, self.cursor)
    }
}
