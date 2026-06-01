use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Pagination<C> {
    Page(PagePagination),
    Cursor(CursorPagination<C>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PagePagination {
    pub current_page: u64,
    pub page_size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CursorPagination<C> {
    pub limit: u64,
    pub has_more: bool,
    pub next_cursor: Option<C>,
}

impl PagePagination {
    pub fn new(current_page: u64, page_size: u64, total_items: Option<u64>) -> Self {
        let total_pages = total_items.map(|total_items| total_items.div_ceil(page_size));

        Self {
            current_page,
            page_size,
            total_pages,
            total_items,
        }
    }
}

impl<C> CursorPagination<C> {
    pub fn new(limit: u64, has_more: bool, next_cursor: Option<C>) -> Self {
        Self {
            limit,
            has_more,
            next_cursor,
        }
    }

    pub fn map_cursor<T>(self, f: impl FnOnce(C) -> T) -> CursorPagination<T> {
        CursorPagination {
            limit: self.limit,
            has_more: self.has_more,
            next_cursor: self.next_cursor.map(f),
        }
    }
}

impl<C> Pagination<C> {
    pub fn page(current_page: u64, page_size: u64) -> Self {
        Self::Page(PagePagination::new(current_page, page_size, None))
    }

    pub fn page_with_total(current_page: u64, page_size: u64, total_items: u64) -> Self {
        Self::Page(PagePagination::new(
            current_page,
            page_size,
            Some(total_items),
        ))
    }

    pub fn cursor(limit: u64, has_more: bool, next_cursor: Option<C>) -> Self {
        Self::Cursor(CursorPagination {
            limit,
            has_more,
            next_cursor,
        })
    }
}
