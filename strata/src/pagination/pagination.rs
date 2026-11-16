use std::num::NonZeroU64;

use serde::{Deserialize, Serialize};

/// 分页响应：页码或游标分页。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Pagination<C> {
    /// 页码分页。
    Page(PagePagination),
    /// 游标分页。
    Cursor(CursorPagination<C>),
}

/// 页码分页信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PagePagination {
    /// 当前页码。
    pub current_page: NonZeroU64,
    /// 每页条数。
    pub page_size: NonZeroU64,
    /// 总页数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u64>,
    /// 总条数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<u64>,
}

/// 游标分页信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CursorPagination<C> {
    /// 查询条数。
    pub limit: NonZeroU64,
    /// 是否还有下一页。
    pub has_more: bool,
    /// 下一页游标。
    pub next_cursor: Option<C>,
}

impl PagePagination {
    /// 构建页码分页信息（按总条数计算总页数）。
    pub fn new(current_page: NonZeroU64, page_size: NonZeroU64, total_items: Option<u64>) -> Self {
        let total_pages = total_items.map(|total_items| total_items.div_ceil(page_size.get()));

        Self {
            current_page,
            page_size,
            total_pages,
            total_items,
        }
    }
}

impl<C> CursorPagination<C> {
    /// 构建游标分页信息。
    pub fn new(limit: NonZeroU64, has_more: bool, next_cursor: Option<C>) -> Self {
        Self {
            limit,
            has_more,
            next_cursor,
        }
    }

    /// 映射游标类型。
    pub fn map_cursor<T>(self, f: impl FnOnce(C) -> T) -> CursorPagination<T> {
        CursorPagination {
            limit: self.limit,
            has_more: self.has_more,
            next_cursor: self.next_cursor.map(f),
        }
    }
}

impl<C> Pagination<C> {
    /// 构造页码分页响应。
    pub fn page(current_page: NonZeroU64, page_size: NonZeroU64) -> Self {
        Self::Page(PagePagination::new(current_page, page_size, None))
    }

    /// 构造含总数的页码分页响应。
    pub fn page_with_total(
        current_page: NonZeroU64,
        page_size: NonZeroU64,
        total_items: u64,
    ) -> Self {
        Self::Page(PagePagination::new(
            current_page,
            page_size,
            Some(total_items),
        ))
    }

    /// 构造游标分页响应。
    pub fn cursor(limit: NonZeroU64, has_more: bool, next_cursor: Option<C>) -> Self {
        Self::Cursor(CursorPagination::new(limit, has_more, next_cursor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nz(value: u64) -> NonZeroU64 {
        NonZeroU64::new(value).unwrap()
    }

    #[test]
    fn page_pagination_computes_total_pages() {
        assert_eq!(
            PagePagination::new(nz(1), nz(10), Some(25)).total_pages,
            Some(3)
        );
        assert_eq!(
            PagePagination::new(nz(1), nz(10), Some(30)).total_pages,
            Some(3)
        );
        assert_eq!(
            PagePagination::new(nz(1), nz(10), Some(20)).total_pages,
            Some(2)
        );
        assert_eq!(PagePagination::new(nz(1), nz(10), None).total_pages, None);
    }

    #[test]
    fn rejects_zero_fields_when_deserializing() {
        for json in [
            r#"{"current_page":0,"page_size":10,"total_pages":null,"total_items":null}"#,
            r#"{"current_page":1,"page_size":0,"total_pages":null,"total_items":null}"#,
        ] {
            assert!(serde_json::from_str::<PagePagination>(json).is_err());
        }
    }

    #[test]
    fn serializes_page_variant_with_type_tag() {
        let pagination = Pagination::<String>::page(nz(1), nz(10));
        let json = serde_json::to_string(&pagination).unwrap();
        assert_eq!(json, r#"{"type":"page","current_page":1,"page_size":10}"#);
    }

    #[test]
    fn serializes_cursor_variant_with_type_tag() {
        let pagination = Pagination::cursor(nz(10), true, Some("abc".to_owned()));
        let json = serde_json::to_string(&pagination).unwrap();
        assert_eq!(
            json,
            r#"{"type":"cursor","limit":10,"has_more":true,"next_cursor":"abc"}"#
        );
    }

    #[test]
    fn rejects_zero_limit_when_deserializing_cursor_pagination() {
        let json = r#"{"limit":0,"has_more":false,"next_cursor":null}"#;

        assert!(serde_json::from_str::<CursorPagination<String>>(json).is_err());
    }

    #[test]
    fn serializes_page_with_totals() {
        let pagination = Pagination::<String>::page_with_total(nz(2), nz(10), 25);
        let json = serde_json::to_string(&pagination).unwrap();
        assert_eq!(
            json,
            r#"{"type":"page","current_page":2,"page_size":10,"total_pages":3,"total_items":25}"#
        );
    }
}
