use serde::Serialize;

use crate::pagination::{CursorPagination, PagePagination};

/// 页码分页结果。
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Paginated<T> {
    /// 数据项。
    pub items: Vec<T>,
    /// 分页信息。
    pub pagination: PagePagination,
}

impl<T> Paginated<T> {
    /// 映射数据项。
    pub fn map_items<U>(self, f: impl FnMut(T) -> U) -> Paginated<U> {
        Paginated {
            items: self.items.into_iter().map(f).collect(),
            pagination: self.pagination,
        }
    }
}

/// 游标分页结果。
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CursorPaginated<T, C> {
    /// 数据项。
    pub items: Vec<T>,
    /// 分页信息。
    pub pagination: CursorPagination<C>,
}

impl<T, C> CursorPaginated<T, C> {
    /// 映射数据项。
    pub fn map_items<U>(self, f: impl FnMut(T) -> U) -> CursorPaginated<U, C> {
        CursorPaginated {
            items: self.items.into_iter().map(f).collect(),
            pagination: self.pagination,
        }
    }

    /// 映射游标类型。
    pub fn map_cursor<U>(self, f: impl FnOnce(C) -> U) -> CursorPaginated<T, U> {
        CursorPaginated {
            items: self.items,
            pagination: self.pagination.map_cursor(f),
        }
    }
}
