/// 默认页码。
pub const DEFAULT_PAGE: u64 = 1;
/// 默认每页条数。
pub const DEFAULT_PAGE_SIZE: u64 = 10;
/// 每页条数上限。
pub const MAX_PAGE_SIZE: u64 = 100;
/// 默认查询条数。
pub const DEFAULT_LIMIT: u64 = 10;
/// 查询条数上限。
pub const MAX_LIMIT: u64 = 100;

/// 页码分页参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paging {
    page: u64,
    page_size: u64,
    need_total: bool,
}

/// 分页参数错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error, strata_error::ErrorInfo)]
pub enum PagingError {
    /// 页码必须大于 0。
    #[error("page must be greater than 0")]
    #[info(kind = "Validation", code = "INVALID_PAGE", message = "页码必须大于 0")]
    InvalidPage,

    /// 每页条数必须大于 0。
    #[error("page size must be greater than 0")]
    #[info(
        kind = "Validation",
        code = "INVALID_PAGE_SIZE",
        message = "每页条数必须大于 0"
    )]
    InvalidPageSize,

    /// 每页条数超过上限。
    #[error("page size must not exceed {MAX_PAGE_SIZE}")]
    #[info(
        kind = "Validation",
        code = "PAGE_SIZE_EXCEEDED",
        message = "每页条数不能超过 {MAX_PAGE_SIZE}"
    )]
    PageSizeExceeded,

    /// 查询条数必须大于 0。
    #[error("limit must be greater than 0")]
    #[info(
        kind = "Validation",
        code = "INVALID_LIMIT",
        message = "查询条数必须大于 0"
    )]
    InvalidLimit,

    /// 查询条数超过上限。
    #[error("limit must not exceed {MAX_LIMIT}")]
    #[info(
        kind = "Validation",
        code = "LIMIT_EXCEEDED",
        message = "查询条数不能超过 {MAX_LIMIT}"
    )]
    LimitExceeded,
}

impl Paging {
    /// 构建分页参数（校验页码与每页条数）。
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

    /// 从可选值构建（缺省使用默认页码与默认每页条数）。
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

    /// 从可选值与自定义默认值构建。
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

    /// 页码。
    pub fn page(&self) -> u64 {
        self.page
    }

    /// 每页条数。
    pub fn page_size(&self) -> u64 {
        self.page_size
    }

    /// 是否需要总数。
    pub fn need_total(&self) -> bool {
        self.need_total
    }
}

/// 游标分页参数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorPaging<C> {
    limit: u64,
    cursor: Option<C>,
}

impl<C> CursorPaging<C> {
    /// 构建游标分页参数（校验查询条数）。
    pub fn new(limit: u64, cursor: Option<C>) -> Result<Self, PagingError> {
        if limit == 0 {
            return Err(PagingError::InvalidLimit);
        }
        if limit > MAX_LIMIT {
            return Err(PagingError::LimitExceeded);
        }

        Ok(Self { limit, cursor })
    }

    /// 从可选值构建（缺省使用默认查询条数）。
    pub fn from_optional(limit: Option<u64>, cursor: Option<C>) -> Result<Self, PagingError> {
        Self::new(limit.unwrap_or(DEFAULT_LIMIT), cursor)
    }

    /// 查询条数。
    pub fn limit(&self) -> u64 {
        self.limit
    }

    /// 游标。
    pub fn cursor(&self) -> Option<&C> {
        self.cursor.as_ref()
    }

    /// 拆分为（条数, 游标）。
    pub fn into_parts(self) -> (u64, Option<C>) {
        (self.limit, self.cursor)
    }
}
