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

#[cfg(test)]
mod tests {
    use crate::pagination::{CursorPagination, PagePagination};

    use super::*;

    #[test]
    fn page_maps_items_keeping_pagination() {
        let paginated = Paginated {
            items: vec![1, 2, 3],
            pagination: PagePagination::new(1, 10, Some(25)),
        };

        let mapped = paginated.map_items(|item| item * 10);

        assert_eq!(mapped.items, vec![10, 20, 30]);
        assert_eq!(mapped.pagination, PagePagination::new(1, 10, Some(25)));
    }

    #[test]
    fn cursor_maps_items_keeping_pagination() {
        let paginated = CursorPaginated {
            items: vec!["a".to_owned(), "b".to_owned()],
            pagination: CursorPagination::new(10, true, Some("next".to_owned())),
        };

        let mapped = paginated.map_items(|item| item.len());

        assert_eq!(mapped.items, vec![1, 1]);
        assert_eq!(mapped.pagination.limit, 10);
        assert_eq!(mapped.pagination.has_more, true);
        assert_eq!(mapped.pagination.next_cursor.as_deref(), Some("next"));
    }

    #[test]
    fn cursor_maps_cursor_type_keeping_items() {
        let paginated = CursorPaginated {
            items: vec![1, 2],
            pagination: CursorPagination::new(10, false, Some(7u64)),
        };

        let mapped = paginated.map_cursor(|cursor| cursor.to_string());

        assert_eq!(mapped.items, vec![1, 2]);
        assert_eq!(mapped.pagination.limit, 10);
        assert_eq!(mapped.pagination.has_more, false);
        assert_eq!(mapped.pagination.next_cursor.as_deref(), Some("7"));
    }
}
